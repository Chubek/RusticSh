pub enum ShAbsyn {
    CompleteCommand(Box<ShAbsyn>, Option<Box<ShAbsyn>>),
    List(Box<ShAbsyn>, Option<SeparatorOp>, Box<ShAbsyn>),
    AndOr(Box<ShAbsyn>, Option<AndOrOp>, Box<ShAbsyn>),
    Pipeline(Box<ShAbsyn>, Option<PipelineOp>, Box<ShAbsyn>),
    Command(Box<ShAbsyn>, Option<RedirectList>),
    CompoundCommand(CompoundCommandType),
    Subshell(Box<ShAbsyn>),
    CompoundList(Box<ShAbsyn>, Option<Separator>, Box<ShAbsyn>),
    Term(Box<ShAbsyn>, Option<SeparatorOp>, Box<ShAbsyn>),
    ForClause(Box<ShAbsyn>, Box<ShAbsyn>),
    Name(String),
    In,
    WordList(Vec<String>),
    CaseClause(String, Option<Box<ShAbsyn>>),
    CaseListNs(Box<ShAbsyn>, Box<ShAbsyn>),
    CaseList(Box<ShAbsyn>, Box<ShAbsyn>),
    CaseItemNs(Box<ShAbsyn>),
    CaseItem(Box<ShAbsyn>),
    Pattern(Box<ShAbsyn>, Option<String>),
    IfClause(Box<ShAbsyn>, Box<ShAbsyn>, Option<Box<ShAbsyn>>),
    ElsePart(Box<ShAbsyn>, Option<Box<ShAbsyn>>),
    WhileClause(Box<ShAbsyn>, Box<ShAbsyn>),
    UntilClause(Box<ShAbsyn>, Box<ShAbsyn>),
    FunctionDefinition(String, Box<ShAbsyn>),
    FunctionBody(CompoundCommandType),
    FName(String),
    BraceGroup(Box<ShAbsyn>),
    DoGroup(Box<ShAbsyn>),
    SimpleCommand(Box<ShAbsyn>, Option<Box<ShAbsyn>>),
    CmdName(String),
    CmdWord(String),
    CmdPrefix(Box<ShAbsyn>, Option<Box<ShAbsyn>>),
    CmdSuffix(Box<ShAbsyn>, Option<Box<ShAbsyn>>),
    RedirectList(Vec<Box<ShAbsyn>>),
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

pub enum CompoundCommandType {
    BraceGroup(Box<ShAbsyn>),
    Subshell(Box<ShAbsyn>),
    ForClause(String, Box<ShAbsyn>, Box<ShAbsyn>),
    CaseClause(String, Box<ShAbsyn>, Box<ShAbsyn>),
    IfClause(Box<ShAbsyn>, Box<ShAbsyn>, Option<Box<ShAbsyn>>),
    WhileClause(Box<ShAbsyn>, Box<ShAbsyn>),
    UntilClause(Box<ShAbsyn>, Box<ShAbsyn>),
    DoGroup(Box<ShAbsyn>),
}

pub enum IoRedirectType {
    IoFile(IoFileType),
    IoHere(IoHereType),
}

pub enum IoFileType {
    Input,
    InputAnd,
    Output,
    OutputAnd,
    OutputAppend,
    InputOutput,
    Clobber,
}

pub enum IoHereType {
    DLess,
    DLessDash,
}

pub enum SeparatorOp {
    Ampersand,
    Semicolon,
}

pub enum PipelineOp {
    Pipe,
    Bang,
}

pub enum AndOrOp {
    AndIf,
    OrIf,
}

pub enum Separator {
    SeparatorOp(SeparatorOp),
    NewlineList(Vec<String>),
}
