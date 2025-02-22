## Capabilities

This service can be used to:

- [x] stat
- [x] read
- [x] write
- [x] create_dir
- [x] delete
- [ ] copy
- [ ] rename
- [ ] ~~list~~
- [ ] scan
- [ ] ~~presign~~
- [ ] blocking

## Configuration

- `root`: Set the working directory of `OpenDAL`
- `connection_string`: Set the connection string of postgres server
- `table`: Set the table of postgresql
- `key_field`: Set the key field of postgresql
- `value_field`: Set the value field of postgresql

## Example

### Via Builder

```rust
use anyhow::Result;
use opendal::services::Postgresql;
use opendal::Operator;

#[tokio::main]
async fn main() -> Result<()> {
    let mut builder = Postgresql::default();
    builder.root("/");
    builder.connection_string("postgresql://you_username:your_password@127.0.0.1:5432/your_database");
    builder.table("your_table");
    // key field type in the table should be compatible with Rust's &str like text
    builder.key_field("key");
    // value field type in the table should be compatible with Rust's Vec<u8> like bytea
    builder.value_field("value");

    let op = Operator::new(builder)?.finish();
    Ok(())
}
```
