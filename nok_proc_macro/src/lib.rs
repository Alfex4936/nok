use proc_macro::{Group, Ident, TokenStream, TokenTree};

fn replace_ident(ident: Ident) -> Option<TokenTree> {
    let ident_str = ident.to_string();

    let new_str = match ident_str.as_str() {
        "실패" => "Err",
        "오케이" => "Ok",
        "문자열" => "String",
        "사전" => "HashMap",
        "기본" => "Default",
        "오류" => "Error",
        "옵션" => "Option",
        "있음" => "Some",
        "없음" => "None",
        "결과" => "Result",
        "출력" => "println",
        "탈출" => "break",
        "비동기" => "async",
        "대기" => "await",
        "무한" => "loop",
        "이동" => "move",
        "크레이트" => "crate",
        "접근_불가_코드" => "unreachable_code",
        "다음_처럼" => "as",
        "상수" => "const",
        "특성" => "trait",
        "위험" => "unsafe",
        "안" => "in",
        "다음에서" => "from",
        "꺼내기" => "unwrap",
        "기본값" => "default",
        "아니" => None?,
        "외부" => "extern",
        "거짓" => "false",
        "함수" => "fn",
        "넣기" => "insert",
        "얻기" => "get",
        "허용" => "allow",
        "안돼" | "종료" | "패닉" => "panic",
        "모듈" => "mod",
        "가변" => "mut",
        "새로운" => "new",
        "다음을_위해" => "for",
        "메인" => "main",
        "공용" => "pub",
        "되돌려주기" => "return",
        "구현" => "impl",
        "레퍼런스" => "ref",
        "매치" => "match",
        "만약" => "if",
        "아니면" => "else",
        "여기서" => "let",
        "정적" => "static",
        "구조" => "struct",
        "예상" => "expect",
        "조건_반복" => "while",
        "사용" => "use",
        "참" => "true",
        "열거" => "enum",
        "구현안됨" => "unimplemented",

        _ => &ident_str,
    };

    let new_ident = Ident::new(new_str, ident.span());
    Some(TokenTree::Ident(new_ident))
}

fn replace_tree(tok: TokenTree, out: &mut Vec<TokenTree>) {
    match tok {
        TokenTree::Group(group) => {
            let mut group_elem = Vec::new();
            replace_stream(group.stream(), &mut group_elem);
            let mut new_stream = TokenStream::new();
            new_stream.extend(group_elem);
            out.push(TokenTree::Group(Group::new(group.delimiter(), new_stream)));
        }
        TokenTree::Ident(ident) => {
            if let Some(ident) = replace_ident(ident) {
                out.push(ident);
            }
        }
        TokenTree::Punct(..) | TokenTree::Literal(..) => {
            out.push(tok);
        }
    }
}

fn replace_stream(ts: TokenStream, out: &mut Vec<TokenTree>) {
    for tok in ts {
        replace_tree(tok, out)
    }
}

#[proc_macro]
pub fn korean(item: TokenStream) -> TokenStream {
    let mut returned = Vec::new();
    replace_stream(item, &mut returned);
    let mut out = TokenStream::new();
    out.extend(returned);
    out
}
