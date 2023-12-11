#[derive(Debug)]
enum Absyn {
    CompleteCommand(Box<Absyn>, Option<Box<Absyn>>),
    List(Box<Absyn>, Option<SeparatorOp>, Box<Absyn>),
    AndOr(Box<Absyn>, Option<AndOrOp>, Box<Absyn>),
    Pipeline(Box<Absyn>, Option<PipelineOp>, Box<Absyn>),
    Command(Box<Absyn>, Option<RedirectList>),
    CompoundCommand(CompoundCommandType),
    Subshell(Box<Absyn>),
    CompoundList(Box<Absyn>, Option<Separator>, Box<Absyn>),
    Term(Box<Absyn>, Option<SeparatorOp>, Box<Absyn>),
    ForClause(String, Box<Absyn>, Box<Absyn>),
    Name(String),
    In,
    WordList(Vec<String>),
    CaseClause(String, Option<Box<Absyn>>),
    CaseListNs(Box<Absyn>, Box<Absyn>),
    CaseList(Box<Absyn>, Box<Absyn>),
    CaseItemNs(Box<Absyn>),
    CaseItem(Box<Absyn>),
    Pattern(Box<Absyn>, Option<String>),
    IfClause(Box<Absyn>, Box<Absyn>, Option<Box<Absyn>>),
    ElsePart(Box<Absyn>, Option<Box<Absyn>>),
    WhileClause(Box<Absyn>, Box<Absyn>),
    UntilClause(Box<Absyn>, Box<Absyn>),
    FunctionDefinition(String, Box<Absyn>),
    FunctionBody(CompoundCommandType),
    FName(String),
    BraceGroup(Box<Absyn>),
    DoGroup(Box<Absyn>),
    SimpleCommand(Box<Absyn>, Option<Box<Absyn>>),
    CmdName(String),
    CmdWord(String),
    CmdPrefix(Box<Absyn>, Option<Box<Absyn>>),
    CmdSuffix(Box<Absyn>, Option<Box<Absyn>>),
    RedirectList(Vec<Box<Absyn>>),
    IoRedirect(IoRedirectType),
    IoFile(IoFileType, String),
    FileName(String),
    IoHere(IoHereType, String),
    HereEnd(String),
    NewlineList(Vec<String>),
    Linebreak(Vec<String>),
    SeparatorOp(char),
    Separator(SeparatorOp, Vec<String>),
    SequentialSep(char, Vec<String>),
}

#[derive(Debug)]
enum CompoundCommandType {
    BraceGroup(Box<Absyn>),
    Subshell(Box<Absyn>),
    ForClause(String, Box<Absyn>, Box<Absyn>),
    CaseClause(String, Box<Absyn>, Box<Absyn>),
    IfClause(Box<Absyn>, Box<Absyn>, Option<Box<Absyn>>),
    WhileClause(Box<Absyn>, Box<Absyn>),
    UntilClause(Box<Absyn>, Box<Absyn>),
    DoGroup(Box<Absyn>),
}

#[derive(Debug)]
enum IoRedirectType {
    IoFile(IoFileType),
    IoHere(IoHereType),
}

#[derive(Debug)]
enum IoFileType {
    Input,
    InputAnd,
    Output,
    OutputAnd,
    OutputAppend,
    InputOutput,
    Clobber,
}

#[derive(Debug)]
enum IoHereType {
    DLess,
    DLessDash,
}

#[derive(Debug)]
enum SeparatorOp {
    Ampersand,
    Semicolon,
}

#[derive(Debug)]
enum PipelineOp {
    Pipe,
    Bang,
}

#[derive(Debug)]
enum AndOrOp {
    AndIf,
    OrIf,
}

#[derive(Debug)]
enum Separator {
    SeparatorOp(SeparatorOp),
    NewlineList(Vec<String>),
}


