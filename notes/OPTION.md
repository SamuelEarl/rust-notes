# OPTION

Let's say you have a struct with some optional values:

```rust
#[derive(Debug, Deserialize, Serialize)]
pub struct Vehicle {
    manufacturer: String,
    model: String,
    year: u32,
    id: Option<String>,
}
```

When you initialize an object from that struct, any optional values have to be either `None` or `Some` and `Some` has to wrap a value. For example, if your struct defines a field as `Option<String>`, then you have to wrap `Some()` around that string value:

```rust
Json::from(Vehicle {
    manufacturer: "Dodge".to_string(),
    model: "RAM 1500".to_string(),
    year: 2021,
    id: Some(Uuid::now_v7().to_string()),
})
```

## Leptos example

Here's another example used in Leptos that shows how to define default values and how to use both custom value and default values:

```rust
#[derive(Clone, Debug)]
pub struct Sizes {
    pub fs: Option<i32>,    // Font size
    pub fw: Option<String>, // Font weight
    pub pv: Option<i32>,    // Padding vertical
    pub ph: Option<i32>,    // Padding horizontal
}

impl Default for Sizes {
    fn default() -> Self {
        Self {
            fs: Some(4),
            fw: Some("normal".to_string()),
            pv: Some(2),
            ph: Some(3),
        }
    }
}
```

This example only uses custom values (no default values):

```html
<Button
    sizes=Some(Sizes {
        fs: Some(5),
        fw: Some("bold".to_string()),
        pv: Some(5),
        ph: Some(5),
    })
>
    "Click"
</Button>
```

This uses some custom values and the rest are default values:

```html
<Button
    sizes=Some(Sizes {
        pv: Some(0),
        ph: Some(0),
        ..Default::default()
    })
>
    "Click"
</Button>
```
