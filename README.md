# OpenVINO XML Operator Extractor

## 概要
OpenVINO IR (.xml) ファイルからオペレータの種類を抽出し、解析するツールです。このツールは、CLI を通じて以下の機能を提供します：

- オペレータの種類を一覧表示
- 各オペレータの出現回数をカウント
- 結果をクリップボードに保存
- 結果をテキストファイルとして保存

## インストール方法

1. Rust のインストール
   Rust がインストールされていない場合は、以下のコマンドでインストールしてください：

   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. リポジトリのクローン
   ```bash
   git clone https://github.com/your-repo/openvino_xml_operator_extractor.git
   cd openvino_xml_operator_extractor
   ```

3. ビルド
   ```bash
   cargo build --release
   ```

## 使い方

以下のコマンドでツールを実行できます：

### 基本的な使い方

```bash
./target/release/openvino_xml_operator_extractor <xml_file>
```

- `<xml_file>`: OpenVINO IR (.xml) ファイルのパス

### オプション

- `-c, --count`: 各オペレータの出現回数を表示
- `-o, --output-dir <dir>`: 結果を保存するディレクトリを指定
- `-b, --clipboard`: 結果をクリップボードに保存

### 使用例

1. オペレータの種類を一覧表示
   ```bash
   ./target/release/openvino_xml_operator_extractor models/example.xml
   ```

2. 各オペレータの出現回数を表示
   ```bash
   ./target/release/openvino_xml_operator_extractor models/example.xml --count
   ```

3. 結果を特定のディレクトリに保存
   ```bash
   ./target/release/openvino_xml_operator_extractor models/example.xml --output-dir results
   ```

4. 結果をクリップボードに保存
   ```bash
   ./target/release/openvino_xml_operator_extractor models/example.xml --clipboard
   ```

## ライセンス
このプロジェクトは MIT ライセンスの下で提供されています。