// https://www.youtube.com/shorts/0bX4oDGbJxE

#[derive(Debug, Clone)]
struct Aluno<'a>
{
    #[allow(unused)]
    nome: &'a str,
    nota: f32,
    status: Option<bool>,
}

fn main()
{
    // ! caracteres que exigem um codigo de escape unicode para funcionarem, como "ã" atrapalham o algoritimo
    let alunos = vec![
        Aluno { nome: "Joao", nota: 8.0, status: None },
        Aluno { nome: "Mariana", nota: 7.0, status: None },
        Aluno { nome: "Jose", nota: 5.0, status: None },
        Aluno { nome: "Pedro", nota: 4.0, status: None },
        Aluno { nome: "Ana", nota: 6.0, status: None },
        Aluno { nome: "Alice", nota: 7.0, status: None }
    ];

    get_status(alunos);
}

fn get_status(alunos: Vec<Aluno>) -> Vec<Aluno<'_>>
{
    let mut last = 0;
    let mut to_print = Vec::<String>::new();

    for i in 0..alunos.len()
    {
        let mut alunos = alunos[i].to_owned();
        alunos.status = Some(alunos.nota >= 6.0);

        let status = alunos.status.into_iter().map(|x| match x
        {
            false => "\x1b[31mfailed\x1b[39m",
            _ => "\x1b[32mpassed\x1b[39m"
        })
        .collect::<Vec<&str>>()[0];
        
        let val = format!("| \x1b[33m{}\x1b[39m: {}", alunos.nome, status);

        if val.len() > last { last = val.len() }

        to_print.push(val.clone());
    }

    let bar = last - 18;

    println!("{}{}{}", "=".repeat(bar / 2), "0", "=".repeat(bar / 2));

    for x in 0..to_print.len()
    {
        println!("{}{}|", to_print[x], " ".repeat(last - to_print[x].len() + 1));
    }

    println!("{}", "=".repeat(bar));

    alunos
}