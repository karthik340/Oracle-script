use obi::{OBIDecode, OBIEncode, OBISchema};
use owasm_kit::{execute_entry_point, ext, oei, prepare_entry_point};

#[derive(OBIDecode, OBISchema)]
struct Input {
    symbols: Vec<String>,
    multiplier: u64,
}

#[derive(OBIEncode, OBISchema)]
struct Result {
    rates: Vec<u64>,
}

const DATA_SOURCE_ID: i64 = 223;
const EXTERNAL_ID: i64 = 0;
fn validate_symbols(symbol_string:String){
  match symbol_string.as_str(){
    "ATOM"=>(),
    "OSMO"=>(),
    "SCRT"=>(),
    "AKT"=>(),
    "UST"=>(),
    "JUNO"=>(),
    "CRO"=>(),
    "ION"=>(),
    "XPRT"=>(),
    "DVPN"=>(),
    "LUNA"=>(),
    "REGEN"=>(),
    "KRT"=>(),
    "IRIS"=>(),
    "IOV"=>(),
    "NGM"=>(),
    "IXO"=>(),
    "BCNA"=>(),
    "BTSG"=>(),
    "XKI"=>(),
    "LIKE"=>(),
    "EEUR"=>(),
    "BAND"=>(),
    "CMDX"=>(),
    "TICK"=>(),
    "MED" =>(),
    "CHEQ"=>(),
    "STARS"=>(),
    "HUAHUA"=>(),
    "LUM" =>(),
    "VDL" =>(),
    "DSM" =>(),
    "XAG"=>(),
    "XAU"=>(),
    "OIL"=>(),
    _=>panic!("invalid symbol_string {}",symbol_string),
  }
}

fn median(arr: &mut Vec<f64>) -> f64 {
  let len_arr = arr.len() as f64;
  if len_arr > 0f64 {
    arr.sort_by(|a,b| a.partial_cmp(b).unwrap());
    let mid = len_arr / 2f64;
    if len_arr as u64 % 2==0{
      (arr[(mid - 1f64) as usize]+arr[mid as usize]) / 2f64
    } else {
      arr[mid as usize]
    }
  }else{
    0f64
  }
}
#[no_mangle]
fn prepare_impl(input: Input) {
  let inputs_dup = input.symbols.clone();
    for symbol in &input.symbols{
      validate_symbols(symbol.to_string());
    }
    oei::ask_external_data(
        EXTERNAL_ID,
        DATA_SOURCE_ID,
        inputs_dup.join(" ").as_bytes(),
    );
}

#[no_mangle]
fn execute_impl(_input: Input) -> Result {
     let mut _exchange_medians:Option<Vec<f64>> = Some(vec![]);
     let raw_input = ext::load_input::<String>(EXTERNAL_ID);
     let mut prices: Vec<Vec<f64>> = vec![vec![]; _input.symbols.len()];
     let inputs:Vec<String> = raw_input.collect();
     if inputs.is_empty(){
       _exchange_medians = None;
     } else {
       for raw in inputs{
         let validator_price_list:Vec<f64> = raw
         .split(',')
         .filter_map(|x| x.parse::<f64>().ok())
         .collect(); 

         for (index,&price) in validator_price_list.iter().enumerate() {
           prices[index].push(price);
         }
       }
       let mut median_prices = vec![0f64; _input.symbols.len()];
       for (idx,price) in prices.iter().enumerate(){
         median_prices[idx]=median(&mut price.to_vec());
       }
       _exchange_medians = Some(median_prices);
     }
     let mut rates:Vec<u64> = Vec::new();
     if _exchange_medians.is_some() {
       let exchange_medians = _exchange_medians.unwrap();
       for item in &exchange_medians {
         rates.push(((*item)*(_input.multiplier as f64)) as u64);
       }
     }
     Result { rates: rates }
}

prepare_entry_point!(prepare_impl);
execute_entry_point!(execute_impl);

