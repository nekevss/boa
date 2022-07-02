use boa_engine::{
    object::{JsArray, JsMap},
    Context, JsResult, JsValue,
};

fn main() -> JsResult<()> {
    // Create a `Context` for the Javascript executor.
    let context = &mut Context::default();

    // Create a new empty map.
    let map = JsMap::new(context);

    // Set a key-value for the map.
<<<<<<< HEAD
    map.set("Key-1", "Value-1", context)?;

    let map_check = map.has("Key-1", context)?;
    assert_eq!(map_check, true.into()); // true

    // Set a second key-value to the same map.
    map.set(2, 4, context)?;

    assert_eq!(map.get_size(context)?, 2.into()); //true

    assert_eq!(map.get("Key-1", context)?, "Value-1".into());
    assert_eq!(map.get(2, context)?, 4.into());
    // Delete an entry with a provided key.
    map.delete("Key-1", context)?;
    assert_eq!(map.get_size(context)?, 1.into());

    let deleted_key_one = map.get("Key-1", context)?;
=======
    map.set(JsValue::new("Key-1"), JsValue::new("Value-1"), context)?;

    let map_check = map.has(JsValue::new("Key-1"), context)?;
    assert_eq!(map_check, JsValue::new(true)); // true

    // Set a second key-value to the same map.
    map.set(JsValue::new("Key-2"), JsValue::new("Value-2"), context)?;

    assert_eq!(map.get_size(context)?, JsValue::new(2)); //true

    let current_key_one = map.get(JsValue::new("Key-1"), context)?;
    assert_eq!(current_key_one, JsValue::new("Value-1"));

    // Delete an entry with a provided key.
    map.delete(JsValue::new("Key-1"), context)?;
    assert_eq!(map.get_size(context)?, JsValue::new(1));

    let deleted_key_one = map.get(JsValue::new("Key-1"), context)?;
>>>>>>> 787313dc88736c4b6592156b2d8a7edc2a837f45

    assert_eq!(deleted_key_one, JsValue::undefined());

    // Retrieve a MapIterator for all entries in the Map.
    let entries = map.entries(context)?;

    let _first_value = entries.next(context)?;

    // Create a multidimensional array with key value pairs -> [[first-key, first-value], [second-key, second-value]]
    let js_array = JsArray::new(context);

    let vec_one = vec![JsValue::new("first-key"), JsValue::new("first-value")];
    let vec_two = vec![JsValue::new("second-key"), JsValue::new("second-value")];

    js_array.push(JsArray::from_iter(vec_one, context), context)?;
    js_array.push(JsArray::from_iter(vec_two, context), context)?;

    // Create a map from the JsArray using it's iterable property.
    let iter_map = JsMap::from_js_iterable(&js_array.into(), context)?;

<<<<<<< HEAD
    assert_eq!(iter_map.get("first-key", context)?, "first-value".into());

    iter_map.set("third-key", "third-value", context)?;
=======
    assert_eq!(
        iter_map.get(JsValue::new("first-key"), context)?,
        "first-value".into()
    );

    iter_map.set(
        JsValue::new("third-key"),
        JsValue::new("third-value"),
        context,
    )?;
>>>>>>> 787313dc88736c4b6592156b2d8a7edc2a837f45

    assert_eq!(iter_map.get_size(context)?, JsValue::new(3));

    Ok(())
}
