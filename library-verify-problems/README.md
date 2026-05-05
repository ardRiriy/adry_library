# library-verify-problems

## nix 下で oj-verify を動かす

初回:

```sh
nix-shell -p python311 rustup gcc pkg-config openssl --run '
  python3 -m venv ~/.cache/adry_library_oj_verify
  ~/.cache/adry_library_oj_verify/bin/pip install "setuptools<81" online-judge-verify-helper
'
```

全件実行:

```sh
nix-shell -p python311 rustup gcc pkg-config openssl --run '
  RUSTUP_TOOLCHAIN=1.85.0 ~/.cache/adry_library_oj_verify/bin/oj-verify run
'
```

単一ファイル実行:

```sh
nix-shell -p python311 rustup gcc pkg-config openssl --run '
  RUSTUP_TOOLCHAIN=1.85.0 ~/.cache/adry_library_oj_verify/bin/oj-verify run /path/to/file
'
```
