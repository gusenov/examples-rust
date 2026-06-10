use rand::RngExt;
use tracing_subscriber::EnvFilter;
use tracing_subscriber::fmt::format::FmtSpan;

#[derive(Debug)]
struct Patient {
    id: uuid::Uuid,
    age: u8,
    gender: String,
    diagnosis_codes: Vec<String>,
}

impl std::fmt::Display for Patient {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Patient({}, age {}, {:?})", self.id, self.age, self.diagnosis_codes)
    }
}

fn generate_patient() {
    let patient_id = uuid::Uuid::new_v4();
    let span = tracing::info_span!("generate_patient", %patient_id);
    let _guard = span.enter();

    tracing::info!("generated patient demographics");
    tracing::info!("generated patient medical history");
    tracing::info!("writing synthetic patient to reference database");

    let demographics = generate_demographics();
    let medical_history = generate_medical_history(&demographics);
    tracing::info!("writing synthetic patient to reference database");
}
// 2026-06-10T08:24:13.025051Z  INFO generate_patient{patient_id=69ae6ba2-6690-42e7-a01e-a9b528f3faa1}: using_tracing: generated patient demographics
// 2026-06-10T08:24:13.025084Z  INFO generate_patient{patient_id=69ae6ba2-6690-42e7-a01e-a9b528f3faa1}: using_tracing: generated patient medical history
// 2026-06-10T08:24:13.025104Z  INFO generate_patient{patient_id=69ae6ba2-6690-42e7-a01e-a9b528f3faa1}: using_tracing: writing synthetic patient to reference database

// 2026-06-10T08:24:13.025191Z  INFO generate_patient{patient_id=69ae6ba2-6690-42e7-a01e-a9b528f3faa1}:generate_demographics: using_tracing: generated patient demographics age=33 gender=F
// 2026-06-10T08:24:13.025229Z  INFO generate_patient{patient_id=69ae6ba2-6690-42e7-a01e-a9b528f3faa1}:generate_demographics: using_tracing: close time.busy=88.8µs time.idle=6.47µs

// 2026-06-10T08:24:13.129105Z  INFO generate_patient{patient_id=69ae6ba2-6690-42e7-a01e-a9b528f3faa1}:generate_medical_history{age=33}: using_tracing: generated patient medical history diagnoses_count=2
// 2026-06-10T08:24:13.129175Z  INFO generate_patient{patient_id=69ae6ba2-6690-42e7-a01e-a9b528f3faa1}:generate_medical_history{age=33}: using_tracing: close time.busy=104ms time.idle=7.42µs

// 2026-06-10T08:24:13.129215Z  INFO generate_patient{patient_id=69ae6ba2-6690-42e7-a01e-a9b528f3faa1}: using_tracing: writing synthetic patient to reference database
// 2026-06-10T08:24:13.129248Z  INFO generate_patient{patient_id=69ae6ba2-6690-42e7-a01e-a9b528f3faa1}: using_tracing: close time.busy=104ms time.idle=12.9µs

#[derive(Debug)]
struct Demographics {
    age: u8,
    gender: String,
}

// it will automatically create a span for that function and attach any arguments as fields
#[tracing::instrument]
fn generate_demographics() -> Demographics {
    // boilerplate code
    //let span = tracing::info_span!("generate_demographics");
    //let _guard = span.enter();

    let age = rand::rng().random_range(0..100);
    let gender = if rand::rng().random_bool(0.5) {
        "M".to_string()
    } else {
        "F".to_string()
    };
    let demographics = Demographics { age, gender };
    tracing::info!(age = demographics.age, gender = %demographics.gender, "generated patient demographics");
    demographics
}

// By default, the #[instrument] macro will also include all the function’s arguments as fields in the span. 
// You can also customise this behaviour by using the fields argument to specify which fields you want to include or exclude.
#[tracing::instrument(fields(age = demographics.age), skip(demographics))]
fn generate_medical_history(demographics: &Demographics) -> Vec<String> {
    std::thread::sleep(std::time::Duration::from_millis(100));

    // Sample from a json file of possible diagnoses based on age but just pretend for now
    let diagnoses = vec!["E11.9".to_string(), "I10".to_string()];
    tracing::info!(
        diagnoses_count = diagnoses.len(),
        "generated patient medical history"
    );
    diagnoses
}

fn main() {
    // This sets up a subscriber that will collect our logs and print them to the terminal in a human readable format.
    //tracing_subscriber::fmt::init();

    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::new("debug"))
        
        // when a span closes, you will get a log line with the duration of the span
        .with_span_events(FmtSpan::CLOSE)

        .init();

    //tracing_subscriber::fmt().json().init();
    
    tracing::info!("Hello, world!");
    // 2026-06-09T16:15:05.609406Z  INFO using_tracing: Hello, world!

    tracing::debug!("This is a debug log");
    // 2026-06-09T16:17:28.508567Z DEBUG using_tracing: This is a debug log

    tracing::error!("database connection failed");

    let host = "localhost";
    let retries = 3;
    let err = "connection refused";
    tracing::error!("database connection failed for host {} after {} retries: {}", host, retries, err);




    // Structured Fields

    // The field names are on the left of the = sign and the values are on the right. 
    // The message string always comes last. 
    // And some of the values have a % symbol in front of them while others do not.

    // % uses the Display trait (i.e. fmt::Display). This is the human-readable representation of a value. It is the same formatting you get when you use {} in a println!.
    // ? uses the Debug trait (i.e. fmt::Debug). This is the programmer-oriented representation of a value. It is the same formatting you get when you use {:?} in a println!.

    let database_url = "postgres://db:5432/reference";
    let patient_id = "a0c7e4a-3f1d-4b5e-9c2f-8d6b3e1a7f04";
    tracing::error!(
        database = %database_url,
        patient_id = %patient_id,
        error = %err,
        "failed to write synthetic patient to reference database"
    );
    // 2026-06-10T07:12:11.388357Z ERROR using_tracing: failed to write synthetic patient to reference database database=postgres://db:5432/reference patient_id=a0c7e4a-3f1d-4b5e-9c2f-8d6b3e1a7f04 error=connection refused

    let age = "";
    tracing::info!(patient_id = %patient_id, age = age, "generated synthetic patient");

    let queue= [1, 2, 3, 4, 5];
    tracing::warn!(queue_depth = queue.len(), "patient write queue length");

    let sql = "";
    let count = 0;
    tracing::debug!(query = %sql, rows_written = count, "batch insert completed");


    let patient = Patient {
        id: uuid::uuid!("9ad9544e-0b74-4e36-b1ff-0c2e6d70837c"),
        age: 67,
        gender: "F".to_string(),
        diagnosis_codes: vec!["E11.9".to_string()],

    };

    // Using Display (%)
    tracing::info!(patient = %patient, "writing synthetic patient to reference database");
    // {"timestamp":"2026-06-10T07:35:39.965813Z","level":"INFO","fields":{"message":"writing synthetic patient to reference database","patient":"Patient(9ad9544e-0b74-4e36-b1ff-0c2e6d70837c, age 67, [\"E11.9\"])"},"target":"using_tracing"}

    // Using Debug (?)
    tracing::info!(patient = ?patient, "writing synthetic patient to reference database");
    // {"timestamp":"2026-06-10T07:35:39.965846Z","level":"INFO","fields":{"message":"writing synthetic patient to reference database","patient":"Patient { id: 9ad9544e-0b74-4e36-b1ff-0c2e6d70837c, age: 67, gender: \"F\", diagnosis_codes: [\"E11.9\"] }"},"target":"using_tracing"}

    // If your variable name is the same as the field name you want, you can omit the field name:
    tracing::info!(?patient, "writing synthetic patient to reference database");




    // Introducing Spans

    generate_patient();
}

