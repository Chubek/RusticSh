enum Ast {
    CompleteCommand(Box<Ast>, Option<Box<Ast>>),
    List(Box<Ast>, Option<SeparatorOp>, Box<Ast>),
    AndOr(Box<Ast>, Option<AndOrOp>, Box<Ast>),
    Pipeline(Box<Ast>, Option<PipelineOp>, Box<Ast>),
    Command(Box<Ast>, Option<RedirectList>),
    CompoundCommand(CompoundCommandType),
    Subshell(Box<Ast>),
    CompoundList(Box<Ast>, Option<Separator>, Box<Ast>),
    Term(Box<Ast>, Option<SeparatorOp>, Box<Ast>),
    ForClause(Box<Ast>, Box<Ast>),
    Name(String),
    In,
    WordList(Vec<String>),
    CaseClause(String, Option<Box<Ast>>),
    CaseListNs(Box<Ast>, Box<Ast>),
    CaseList(Box<Ast>, Box<Ast>),
    CaseItemNs(Box<Ast>),
    CaseItem(Box<Ast>),
    Pattern(Box<Ast>, Option<String>),
    IfClause(Box<Ast>, Box<Ast>, Option<Box<Ast>>),
    ElsePart(Box<Ast>, Option<Box<Ast>>),
    WhileClause(Box<Ast>, Box<Ast>),
    UntilClause(Box<Ast>, Box<Ast>),
    FunctionDefinition(String, Box<Ast>),
    FunctionBody(CompoundCommandType),
    FName(String),
    BraceGroup(Box<Ast>),
    DoGroup(Box<Ast>),
    SimpleCommand(Box<Ast>, Option<Box<Ast>>),
    CmdName(String),
    CmdWord(String),
    CmdPrefix(Box<Ast>, Option<Box<Ast>>),
    CmdSuffix(Box<Ast>, Option<Box<Ast>>),
    RedirectList(Vec<Box<Ast>>),
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

enum CompoundCommandType {
    BraceGroup(Box<Ast>),
    Subshell(Box<Ast>),
    ForClause(String, Box<Ast>, Box<Ast>),
    CaseClause(String, Box<Ast>, Box<Ast>),
    IfClause(Box<Ast>, Box<Ast>, Option<Box<Ast>>),
    WhileClause(Box<Ast>, Box<Ast>),
    UntilClause(Box<Ast>, Box<Ast>),
    DoGroup(Box<Ast>),
}

enum IoRedirectType {
    IoFile(IoFileType),
    IoHere(IoHereType),
}

enum IoFileType {
    Input,
    InputAnd,
    Output,
    OutputAnd,
    OutputAppend,
    InputOutput,
    Clobber,
}

enum IoHereType {
    DLess,
    DLessDash,
}

enum SeparatorOp {
    Ampersand,
    Semicolon,
}

enum PipelineOp {
    Pipe,
    Bang,
}

enum AndOrOp {
    AndIf,
    OrIf,
}

enum Separator {
    SeparatorOp(SeparatorOp),
    NewlineList(Vec<String>),
}

