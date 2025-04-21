use clap::{Arg, Command};
use clipboard::{ClipboardContext, ClipboardProvider};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, Write};
use xmltree::{Element, XMLNode};

/// エラーハンドリング用のエイリアス
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

/// XMLファイルを読み込み、<layer> 要素の type 属性を抽出する
fn extract_operator_types(path: &str) -> Result<Vec<String>> {
    // ファイルを開いてバッファリーダーに包む
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    // XML をパース
    let root = Element::parse(reader)?;
    let mut ops = Vec::new();

    // <layers> 以下の <layer> 要素を再帰的に探索
    fn traverse(elem: &Element, ops: &mut Vec<String>) {
        if elem.name == "layer" {
            if let Some(op_type) = elem.attributes.get("type") {
                ops.push(op_type.clone());
            }
        }
        // 子要素を探索
        for child in &elem.children {
            if let XMLNode::Element(child_elem) = child {
                traverse(child_elem, ops);
            }
        }
    }

    traverse(&root, &mut ops);
    Ok(ops)
}

/// ベクタ内の文字列の出現回数をカウントして HashMap にまとめる
fn count_ops(ops: &[String]) -> HashMap<String, usize> {
    let mut counts = HashMap::new();
    for op in ops {
        *counts.entry(op.clone()).or_insert(0) += 1;
    }
    counts
}

fn main() -> Result<()> {
    // CLI 引数定義
    let matches = Command::new("extract_operators")
        .version("1.0")
        .author("Your Name <you@example.com>")
        .about("OpenVINO IR .xml からオペレータ種別を抽出・解析します")
        .arg(
            Arg::new("xml_file")
                .help("IR の .xml ファイルパス")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::new("count")
                .short('c')
                .long("count")
                .help("各オペレータの出現回数を表示")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("output_dir")
                .short('o')
                .long("output-dir")
                .help("出力先ディレクトリを指定")
                .value_parser(clap::value_parser!(String)),
        )
        .arg(
            Arg::new("clipboard")
                .short('b')
                .long("clipboard")
                .help("結果をクリップボードに保存")
                .action(clap::ArgAction::SetTrue),
        )
        .get_matches();

    let xml_path = matches.get_one::<String>("xml_file").unwrap();
    let output_dir = matches.get_one::<String>("output_dir");
    let ops = extract_operator_types(xml_path)?;

    if ops.is_empty() {
        println!("指定された XML からオペレータが見つかりませんでした。");
        return Ok(());
    }

    let clipboard_mode = matches.get_flag("clipboard");

    if matches.get_flag("count") {
        let counts = count_ops(&ops);
        let mut items: Vec<_> = counts.into_iter().collect();
        items.sort_by_key(|(op, _)| op.clone());
        let result = items
            .iter()
            .map(|(op, cnt)| format!("{}: {}", op, cnt))
            .collect::<Vec<_>>()
            .join("\n");

        if clipboard_mode {
            let mut ctx: ClipboardContext = ClipboardProvider::new()?;
            ctx.set_contents(result.clone())?;
            println!("結果をクリップボードに保存しました。");
        } else if let Some(dir) = output_dir {
            let output_path = format!(
                "{}/{}_counts.txt",
                dir,
                xml_path.rsplit('/').next().unwrap().replace(".xml", "")
            );
            let mut file = File::create(output_path)?;
            writeln!(file, "{}", result)?;
        } else {
            println!("{}", result);
        }
    } else {
        let mut unique_ops: Vec<_> = ops.into_iter().collect();
        unique_ops.sort();
        unique_ops.dedup();
        let result = unique_ops.join("\n");

        if clipboard_mode {
            let mut ctx: ClipboardContext = ClipboardProvider::new()?;
            ctx.set_contents(result.clone())?;
            println!("結果をクリップボードに保存しました。");
        } else if let Some(dir) = output_dir {
            let output_path = format!(
                "{}/{}.txt",
                dir,
                xml_path.rsplit('/').next().unwrap().replace(".xml", "")
            );
            let mut file = File::create(output_path)?;
            writeln!(file, "{}", result)?;
        } else {
            println!("{}", result);
        }
    }

    Ok(())
}
