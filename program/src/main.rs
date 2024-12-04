use guest::{read_models, PublicValuesStruct};

pub fn main() {
    let scaler_path = "./guest/src/model/scaler_params.json";
    let linear_model_path = "./guest/src/model/linear_regression_params.json";
    let ridge_model_path = "./guest/src/model/ridge_regression_params.json";
    let poly_ridge_model_path = "./guest/src/model/polynomial_ridge_regression_params.json";

    // Read the test dataset
    let Ok((test_features, actual_amounts)) = read_test_dataset() else {
        todo!()
    };
    // println!("test: {:?}", test_features);
    // Read the models
    let Ok((scaler, linear_model, ridge_model, poly_ridge_model)) = read_models(
        scaler_path,
        linear_model_path,
        ridge_model_path,
        poly_ridge_model_path,
    ) else {
        eprintln!(
            "Error reading models: {:?}",
            read_models(
                scaler_path,
                linear_model_path,
                ridge_model_path,
                poly_ridge_model_path
            )
        );
        return; // Exit or take alternative action
    };

    let Ok(x) = flatten(test_features) else {
        todo!()
    };
    let test = 3.32;

    let model_input = ModelInput {
        // test_features,
        // actual_amounts,
        test,
        scaler,
        ridge_model,
        x,
    };

    // let (prove, verify) = guest::build_int_to_string();
    //
    // let (output, proof) = prove(81);
    // let is_valid = verify(proof);
    //
    // println!("int to string output: {:?}", output);
    // println!("int to string valid: {}", is_valid);
    //
    // let (prove, verify) = guest::build_string_concat();
    //
    // let (output, proof) = prove(20);
    // let is_valid = verify(proof);
    //
    // println!("string concat output: {:?}", output);
    // println!("string concat valid: {}", is_valid);
    let scaler_path = sp1_zkvm::io::read::<str>();
    let linear_model_path = sp1_zkvm::io::read::<str>();
    let ridge_model_path = sp1_zkvm::io::read::<str>();
    let poly_ridge_model_path = sp1_zkvm::io::read::<str>();

    let (scaler, linear_model, ridge_model, poly_ridge_model) = read_models(
        scaler_path,
        linear_model_path,
        ridge_model_path,
        poly_ridge_model_path,
    );
    let bytes = PublicValuesStruct::abi_encode(&PublicValuesStruct {
        scaler_path,
        linear_model_path,
        ridge_model_path,
        poly_ridge_model_path,
        scaler,
        linear_model,
        ridge_model,
        poly_ridge_model,
    });

    sp1_zkvm::io::commit_slice(&bytes);
}
