## 前提
全ての操作はsrc-pythonで行う
```
cd src-python
```

## 環境構築

### 仮想環境作成(初回のみ)
```
python -m venv .myenv
```


### 仮想環境有効
```
. ./.myenv/bin/activate
```


### パッケージインストール(初回のみ)
```
pip install -r requirements.txt
```



## ファームウェアファイル格納
下記ディレクトリにファイルを格納
src-python/resources/firmware



## 実行ファイル作成

### ビルド(specファイル作成)
```
pyinstaller -y --onefile \
    --add-data "resources/targets:esptool/targets" \
    --add-data "resources/firmware:esptool/firmware" \
    --add-data "config.json:." \
    src/esptool_ex.py
```

### ビルド(作成済みのspecファイルを使用)
```
pyinstaller esptool_ex.spec -y
```


### 実行ファイルをコピー
Mac(arm)
```
cp dist/esptool_ex ../src-tauri/sidecar-app/esptool_ex-aarch64-apple-darwin
```


### 仮想環境無効
```
deactivate
```
