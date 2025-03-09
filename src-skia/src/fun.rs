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
        match self.status
        {
            Some(true) => "\x1b[32mpassed\x1b[39m".to_owned(),
            Some(false) => "\x1b[31mfailed\x1b[39m".to_owned(),
            None =>
            {
                self.status = Some(self.nota >= 6.0);
                return self.get_status()
            }
        }
    }
}

fn main()
{
    // ! caracteres que exigem um codigo de escape unicode para funcionarem, como "ã" atrapalham o algoritimo
    let alunos = vec![
        Aluno::new("Joao", 8.0, None),
        Aluno::new("Mariana", 7.0, None),
        Aluno::new("Jose", 5.0, None),
        Aluno::new("Pedro", 4.0, None),
        Aluno::new("Ana", 6.0, None),
        Aluno::new("Alice", 7.0, None),
        Aluno::new("Valentina", 8.0, None),
    ];

    print_status(alunos);

    let produtos = vec![
        Product { name: "leite", price: 32 },
        Product { name: "ovo", price: 64 },
        Product { name: "licor", price: 100 },
        Product { name: "mel", price: 59 },
    ];

    println!("{:#?}", filtrar(produtos, 64, 31));
}

fn print_status(alunos: Vec<Aluno>) -> Vec<Aluno<'_>>
{
    let mut last  = 0;
    let mut to_print = Vec::<String>::new();

    for mut aluno in alunos.to_owned()
    {
        let status = aluno.get_status();
        
        let val = format!("| \x1b[33m{}\x1b[39m: {}", aluno.nome, status);

        last = last.max(val.len());

        to_print.push(val.clone());
    }

    let bar = last - 18;
    let top = format!("{}", "=".repeat(bar / 2));

    print!("\x1B[2J\x1B[H\x1B[?25l");
    println!("Lista de Status:\n{}{}{}", top, "\u{2764}", top);

    for line in to_print
    {
        println!("{}{}|", line, " ".repeat(last - line.len() + 1));
    }

    println!("{}\x1B[?25h", "=".repeat(bar));

    alunos
}

// https://www.youtube.com/shorts/oBEZiQaRZao

#[derive(Debug, Clone)]
struct Product<'a>
{
    #[allow(unused)]
    name: &'a str,
    price: usize,
}

fn filtrar(list: Vec<Product>, max: usize, min: usize) -> Vec<Product>
{
    list.into_iter().filter(|x| x.price > min && x.price < max).collect()
}