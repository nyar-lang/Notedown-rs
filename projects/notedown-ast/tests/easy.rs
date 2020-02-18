use notedown_ast::{parse, ToHTML};

#[test]
fn test_text() {
    let f = parse(
        r#"
        Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor
        incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis
        nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat.
        Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore
        eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt
        in culpa qui officia deserunt mollit anim id est laborum.
    "#,
    );
    // println!("{:#?}", f);
    println!("{}", f.free().to_html_default());
}

#[test]
fn test_style() {
    let f = parse(
        r#"
        *斜体 Italic*

        **粗体 Bold**

        ***斜粗体 Bold-Italic***

        ~下划线 Underline~

        ~~删除线 Strikethrough~~

        ~~~数据删除 Undercover~~~

        `代码 code`

        ``code`escape``
    "#,
    );
    // println!("{:#?}", f);
    println!("{}", f.free().to_html_default());
}

#[test]
fn test_math() {
    let f = parse(
        r#"
        $x$ $$x^2$$

        $$x^2$$ $x$

        $$x^2$$ $$x^2$$

        $$x^4$$
    "#,
    );
    // println!("{:#?}", f);
    println!("{}", f.free().to_html_default());
    unreachable!()
}

#[test]
fn test_function() {
    let input = parse(
        r#"
        \netease[song][22724727]
        \netease{id = 28629058, type = "song", autoplay = true}
        \unknow[arg1][arg2]{arg3 = "string", "arg4" = 42}
    "#,
    );
    // println!("{:#?}", f);
    println!("{}", input.free().to_html_default());
}

#[test]
fn test_table() {
    let input = parse(
        r#"
         |name | age | level
         :-|-:
         aster | 17
    "#,
    );
    // println!("{:#?}", f);
    println!("{}", input.free().to_html_default());
}

#[test]
fn test_code() {
    let input = parse(
        r#"
        ```js
        let r = 0
        ```
    "#,
    );
    // println!("{:#?}", f);
    println!("{}", input.free().to_html_default());
}
