## bugs
- tikv#1768

bug location: 
`components/pd_client/src/util.rs`
```
184         match try_connect_leader(
185             Arc::clone(&self.inner.rl().env),
186             Arc::clone(&self.inner.rl().security_mgr),
187             self.inner.rl().members.clone()) {
188             Ok((cli, mbrs)) => {
189                 let mut inner = self.inner.wl();

```

- tikv#2493

bug location:
`components/pd_client/src/util.rs`
```
350         match func(&client.inner.rl().client_stub).map_err(Error::Grpc) {
200             let mut inner = self.inner.wl();

```
