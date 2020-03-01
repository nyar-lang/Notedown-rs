use notedown_ast::{parse, ToHTML};

fn trim_eq(input: &str, output: &str) {
    let input = parse(input).to_html().replace(" ", "").replace("\n", "");
    let output = output.replace(" ", "").replace("\n", "");
    assert_eq!(input, output);
}

#[test]
fn test_text() {
    let input = r#"
        Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor
        incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis
        nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat.
        Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore
        eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt
        in culpa qui officia deserunt mollit anim id est laborum.
    "#;
    let output = r#"
        <p>
            Loremipsumdolorsitamet,consecteturadipiscingelit,seddoeiusmodtempor</br>
            incididuntutlaboreetdoloremagnaaliqua.Utenimadminimveniam,quis</br>
            nostrudexercitationullamcolaborisnisiutaliquipexeacommodoconsequat.</br>
            Duisauteiruredolorinreprehenderitinvoluptatevelitessecillumdolore</br>
            eufugiatnullapariatur.Excepteursintoccaecatcupidatatnonproident,sunt</br>
            inculpaquiofficiadeseruntmollitanimidestlaborum.
        </p>
    "#;
    trim_eq(input, output)
}

#[test]
fn test_style() {
    let input = r#"
        *斜体 Italic*

        **粗体 Bold**

        ***斜粗体 Bold-Italic***

        ~下划线 Underline~

        ~~删除线 Strikethrough~~

        ~~~数据删除 Undercover~~~

        `代码 code`

        ``code`escape``
    "#;
    let output = r#"
        <p><i>斜体Italic</i></p>
        <p><b>粗体Bold</b></p>
        <p><b><i>斜粗体Bold-Italic</i></b></p>
        <p><u>下划线Underline</u></p>
        <p><del>删除线Strikethrough</del></p>
        <p><spanclass="undercover">数据删除Undercover</span></p>
        <p><code>代码code</code></p>
        <p><code>code`escape</code></p>
    "#;
    trim_eq(input, output)
}

#[test]
fn test_math() {
    let input = r#"
        $x$ $$x^2$$

        $$x^2$$ $x$

        $$x^2$$ $$x^2$$

        $$x^4$$
    "#;
    let output = r#"
        <p><spanclass="math">$x$</span><spanclass="math">$\displaystyle{x^2}$</span></p>
        <p><spanclass="math">$\displaystyle{x^2}$</span><spanclass="math">$x$</span></p>
        <p><spanclass="math">$\displaystyle{x^2}$</span><spanclass="math">$\displaystyle{x^2}$</span></p>
        <pclass="math">$$x^4$$</p>
    "#;
    trim_eq(input, output)
}

#[test]
fn test_function() {
    let input = r#"
        \netease[song][22724727]
        \netease{id = 28629058, type = "song", autoplay = true}
        \unknow[arg1][arg2]{arg3 = "string", "arg4" = 42}
        \title: Test
        \date: 2018-01-08
    "#;
    let output = r#"
        <meting-jsserver="netease"type="song"id="22724727"></meting-js>
        <meting-jsserver="netease"type="song"id="28629"autoplay="true"></meting-js>
        <pre>\unknow{"arg1","arg2",arg3="string",arg4="42"}</pre>
    "#;
    trim_eq(input, output)
}

#[test]
fn test_table() {
    let input = r#"
        |name | age | level
        :-|-:
        aster | 17
    "#;
    let output = r#"
        <table>
            <thead><thalign="left">name</th><thalign="right">age</th><thalign="right">level</th></thead>
            <tbody><tr><tdalign="left">aster</td><tdalign="right">17</td><tdalign="right"></td></tr></tbody>
        </table>
    "#;
    trim_eq(input, output)
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
    println!("{}", input.to_html());
}

#[test]
fn test_quote() {
    let input = r#"
        > - 1
        >   2
        > - 3
    "#;
    let output = r#"
        <blockquote><ul><li><p>1</br>  2</p></li><li><p>3</p></li></ul></blockquote>
    "#;
    trim_eq(input, output)
}

#[test]
fn test_escaping() {
    let input = r#"
     \\
    "#;
    let output = r#"
        <p>\</p>
    "#;
    trim_eq(input, output)
}
