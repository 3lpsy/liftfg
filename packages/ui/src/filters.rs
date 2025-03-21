use dioxus::{
    hooks::use_memo,
    signals::{Memo, Readable, Signal},
};
use fgdb::data::workout::WorkoutData;

pub fn use_workouts_searched<T>(
    workouts_ctx: T,
    search_sig: Signal<String>,
) -> Memo<Vec<WorkoutData>>
where
    T: Readable<Target = Vec<WorkoutData>> + 'static,
{
    use_memo(move || {
        let workouts = workouts_ctx.read().clone();
        let search = search_sig.read().clone();
        search_workouts(&workouts, &search)
            .into_iter()
            .cloned()
            .collect::<Vec<WorkoutData>>()
    })
}

pub fn search_workouts<'a>(workouts: &'a [WorkoutData], query: &str) -> Vec<&'a WorkoutData> {
    if query.trim().is_empty() {
        return workouts.iter().collect();
    }
    let query = query.to_lowercase();
    let query_tokens: Vec<&str> = query.split_whitespace().collect();
    let mut scored_results: Vec<(f32, &'a WorkoutData)> = Vec::new();
    for workout in workouts {
        let mut score = 0.0;
        let workout_name = workout.name.to_lowercase();
        if workout_name == query {
            score += 10.0;
        }
        let workout_tokens: Vec<&str> = workout_name.split_whitespace().collect();
        for q_token in &query_tokens {
            if workout_tokens.contains(q_token) {
                score += 5.0;
            }
            for w_token in &workout_tokens {
                if w_token.contains(q_token) {
                    score += 2.0;
                }
                if w_token.starts_with(q_token) {
                    score += 1.0;
                }
            }
        }
        if let Some(muscles) = &workout.workout_muscle {
            for wm in muscles {
                if let Some(muscle) = &wm.muscle {
                    let muscle_name = muscle.name.to_lowercase();
                    let muscle_long_name = muscle.long_name.to_lowercase();
                    for q_token in &query_tokens {
                        if muscle_name.contains(q_token) || muscle_long_name.contains(q_token) {
                            score += 3.0;
                        }
                    }
                }
            }
        }
        if score > 0.0 {
            scored_results.push((score, workout));
        }
    }
    scored_results.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap_or(std::cmp::Ordering::Equal));
    scored_results
        .into_iter()
        .map(|(_, workout)| workout)
        .collect()
}
