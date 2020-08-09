## bugs

- servo#13388

(commit 6fbd2aa5b7628bd47971806ddf438cd350a60bee fixes the same bug)

bug location:
`components/net/fetch/methods.rs`
```
471         if let ResponseBody::Done(ref vec) = *response.body.lock().unwrap() {
474             assert!(*response.body.lock().unwrap() == ResponseBody::Empty);

```

- 2952ccfae2e4883efc4886988dcf17d07d89f66c

bug location:
`components/script/dom/cssrulelist.rs`
```
124     let mut guard = parent_stylesheet.shared_lock.write();
128         let dom_rule = CSSRule::new_specific(&window, parent_stylesheet, new_rule);
```
 
- servo#11129

bug location:
`components/net/http_loader.rs`
```
1614                 let mut completed_body = res_body.lock().unwrap();
1617                     *res_body.lock().unwrap() = ResponseBody::Done((*body).clone());
```

- fdb1e511bde3fa9e3b3c524a07687dc52131fa0b 

bug location:
`components/constellation/constellation.rs`
```
 657     pub constellation_chan: Arc<Mutex<Sender<FromCompositorMsg>>>,
 664             constellation_chan: Arc::new(Mutex::new(constellation_chan)),

 685             let chan = self
 686                 .constellation_chan
 687                 .lock()
 688                 .unwrap_or_else(|err| err.into_inner());
 689             let _ = chan.send(msg);
 690         }

```

- (bug related to 1e380137831eaf94a1f602c9d8dfae08f10893fa is not reporduced here because of API change)

## misc
Before building, run:
```
sudo apt install python-virtualenv python-pip
./mach bootstrap
```

The project is build by `./mach`. You can replace `python/servo/devenv_commands.py:status = self.run_cargo_build_like_command("check", params, env=env, features=features, **kwargs)` 's "check" with customized commands, then `$ ./mach check` to run the detector.
