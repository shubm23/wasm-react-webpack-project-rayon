
use wasm_bindgen::prelude::*;
use rayon::prelude::*;
use rayon::current_num_threads;
use web_sys::{console,window,Response};
use wasm_bindgen_futures::JsFuture;
use js_sys::{Array, Promise};
use futures::future::join_all;
use wasm_bindgen::JsCast;

fn is_prime(n: i32) -> bool {
    if n < 2 {
        return false;
    }
    for i in 2..=((n as f64).sqrt() as i32) {
        if n % i == 0 {
            return false;
        }
    }
    true
}

fn average(arr: &[i32]) -> f64 {
    arr.iter().sum::<i32>() as f64 / arr.len() as f64
}

fn is_part_of_sequence(i: usize, arr: &[i32]) -> bool {
    (i > 0 && arr[i] == arr[i - 1] + 1) ||
        (i + 1 < arr.len() && arr[i] == arr[i + 1] - 1)
}

#[wasm_bindgen]
pub fn complex_filter(arr: &[i32]) -> Vec<i32> {
    let avg = average(arr);
    arr.par_iter()
        .enumerate()
        .filter(|&(ref i, &x)| is_prime(x) && (x as f64 > avg) && !is_part_of_sequence(*i, arr))
        .map(|(_, &x)| x)
        .collect()

}

#[wasm_bindgen]
pub fn parallel_wasm(n: i32) -> JsValue {
    let max_n: u32 = 1_000_000;
    let n = n.max(0).min(max_n as i32) as u32;
    let thread = current_num_threads();
    
    let performance = window().unwrap().performance().unwrap();
    let start = performance.now();
    
    let data: Vec<u32> = (1..=n).collect();
    
    let result: Vec<u64> = data.par_iter()
        .map(|n| (*n as u64) * (*n as u64))
        .collect();
    
    let js_array = Array::new();
    for val in result {
        js_array.push(&JsValue::from_f64(val as f64));
    }
    
    // End time
    let end = performance.now();
    // Log to console
    let msg = format!(
        "Processed {} items in WASM in {:.2} ms, no of thread {}",
        js_array.length(),
        end - start,
        thread
    );
    console::log_1(&msg.into());
    
    JsValue::from(js_array)
}

#[wasm_bindgen]
pub fn fetch_parallel(n: i32) -> Promise {
    let max_n = n.max(0);
    let thread = current_num_threads();
    // Start a performance timer
    let perf = window().unwrap().performance().unwrap();
    let start = perf.now();
    
    
    let urls: Vec<String> = (1..=max_n)
        .map(|i| format!("https://jsonplaceholder.typicode.com/todos/{}", i))
        .collect();
    
    let futures = urls.into_iter().map(fetch_json);
    let joined = join_all(futures);
    
    wasm_bindgen_futures::future_to_promise(async move {
        let results = joined.await;
        let js_array = Array::new();
        
        for res in results {
            match res {
                Ok(text) => js_array.push(&JsValue::from_str(&text)),
                Err(_) => js_array.push(&JsValue::from_str("Error")),
            };
        }
        
        // End time
        let end = window().unwrap().performance().unwrap().now();
        let duration = end - start;
        
        // Log to the browser console
        console::log_1(&format!(
            "Fetched {} APIs in {:.2} ms (WASM) threads used {}",
            max_n, duration,thread
        ).into());
        
        Ok(js_array.into())
    })
}

async fn fetch_json(url: String) -> Result<String, JsValue> {
    let win = window().unwrap();
    let resp_value = JsFuture::from(win.fetch_with_str(&*url)).await?;
    
    let resp: Response = resp_value.dyn_into()?;
    let text = JsFuture::from(resp.text()?).await?;
    Ok(text.as_string().unwrap_or_else(|| "No text".to_string())) // ✅ returns owned value
}