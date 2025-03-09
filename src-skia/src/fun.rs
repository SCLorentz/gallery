// https://www.youtube.com/shorts/0bX4oDGbJxE

#[derive(Debug, Clone)]
struct Aluno<'a>
{
    #[allow(unused)]
    nome: &'a str,
    nota: f32,
    status: Option<bool>,
}

impl Aluno<'_>
{
    pub fn new(nome: &'static str, nota: f32, status: Option<bool>) -> Self
    {
        Self { nome, nota, status }
    }

    pub fn get_status(&mut self) -> String
    {
        if self.status.is_none()
        {
            self.status = Some(self.nota >= 6.0)
        }

        self.status.into_iter().map(|x| match x
        {
            false => "\x1b[31mfailed\x1b[39m",
            _ => "\x1b[32mpassed\x1b[39m"
        })
        .collect::<Vec<&str>>()[0].to_owned()
    }
}

/*struct Box
{
    size: usize,
    content: Vec<&'static str>, // each value inside vec represents one line
}

impl Box
{
    pub fn new(size: usize, content: Vec<&'static str>) -> Self
    {
        Self { size, content }
    }

    pub print()
}*/

fn main()
{
    // ! caracteres que exigem um codigo de escape unicode para funcionarem, como "ã" atrapalham o algoritimo
    let alunos = vec![
        Aluno::new("Joao", 8.0, None),
        Aluno::new("Mariana", 7.0, None),
        Aluno::new("Jose", 5.0, None),
        Aluno::new("Pedro", 4.0, None),
        Aluno::new("Ana", 6.0, None),
        Aluno::new("Alice", 7.0, None)
    ];

    print_status(alunos);
}

fn print_status(alunos: Vec<Aluno>) -> Vec<Aluno<'_>>
{
    let mut last = 0;
    let mut to_print = Vec::<String>::new();

    for i in 0..alunos.len()
    {
        let mut aluno = alunos[i].to_owned();

        let status = aluno.get_status();
        
        let val = format!("| \x1b[33m{}\x1b[39m: {}", aluno.nome, status);

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