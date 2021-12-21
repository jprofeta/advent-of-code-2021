pub const TEST_INPUT: &str = "\
[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]
";

pub const TEST_RESULT_PART1: i32 = 26397;
pub const TEST_RESULT_PART2: i32 = 288957;

pub const PUZZLE_INPUT: &str = "\
{<{[<((<[<({<{(){}}(()())>[<<><>>((){})]}(<<{}{}>(<><>)>))(<[(<>{}){()()}]{{{}[]}(()())}>[
((<{[<([<<{[<(()())([]())>)<<<{}{}>{[][]}>{<<>>(()[])}>}<(((()[])[{}<>])[<[]{}>{{}[]}])[[({}{})
({(<[([((<(<[(<>{}){()[]}]<[[]<>]<(){}]>><{(<>())[<>{}]}{(<>[])(<>{})}>)([(<(){}>[(){}]){{()[]}{<>{}}}][<<[]
{{{[(([{{{[{[(()())[<>[]]]{([]{}){()<>}}}]{{{((){})[[][]]}[[{}{}]<[]()>]}(<(<>()){{}{}>>([{}()
[<{{{[[(<[(([<{}[]>{()()}][[<>{}]]))]([<<{{}[]}>>([(<>[])<(){}>][[{}{}](()())])]{[{[<><>]}((<>{})[
{<[<[[<<(<<<<(<>[])(<>[])>{([]())}>[{{[]()}}<({}())[<>()]>]>{[((<>{})[[]<>])([()<>]{<>()})](((<>()
([({({[{(([<[<[]>]>]<<({[]()}<[]<>>)[[()[]]{[]{}}]>>))((<<{<<><>>}{(<>{})<[]()>}>([[<>{}]<<>{}>
{(<([{{{{(((({{}[]}))){[(({}<>))({<><>}<[][]])][<<[]{}>[{}{}]>{<[]>{[]{}}}]})({[{([]<>)([]<>)}
{([{{{(([<[[[<(){}>{()[]}][([]<>)<[]{}>]]<{(<>())[[]()]}[{(){}}[()<>]]>]([({[]{}}{()[])){{<>()}{()[]}}])>[<[
[([{<{(<({<{[[<>{}][<>[]]]<{{}<>}<{}{}>>}[({()[]])[[<>()]({}<>)]]>{<([{}<>]<[]<>>)[{[]()}{{}[]}]>[[{[][]
{{<(<{(<[<(((<[]<>><{}[]>))({(<>{}){[][]}}([[]()]({}<>))>)>[((((()<>)<<>[]>){(<>{}){<>{}}})(<({}(
{{[{<{<([{[<{[{}{}]{(){}}}{{[]<>}<()[]>}>]<[({[][]}<{}<>>)(<{}{}><[]()>)]<[{<>{}}(()())]([[]{}]<[]<>>)>>>[{[{
<(([<<{{<(([(({}{})[[]{}])[<{}>(<>{})]]<([{}{}]{()[]})>))({(<(()<>)<<><>>>(([]<>)))}<<<[()
<<<(({{([{({({()()}<<>[]>)<{()[]}[<>()>>}){[<[<>{}][<><>]>[{()[]}[<><>]]]<<[<>[]]{<>()}>>}}[(<[[(){}](()
<<[[{{([(([([[<>{}][(){}]])]{({{()}(<>{})}{{{}<>}{<><>}})(([<><>])({{}[]}<<><>>))})([{({()<>}<[][]>)}]))
[(<<<<<{[<({[{{}[]}<()<>>]<[{}[]][[]<>]>})>[<<{(()())({}<>)}{({}())(<>[]]}><<<{}<>>([]<>)><[
<<({({{([(<([[{}()]<[]{}>]<<<>()>>)><{(<{}<>><{}()>)<{(){}}[<>]>}{([[]{}][[]<>])<({}()){{}[
{<{([[[[[{<<[[{}{}]([]<>)]{{{}()}[<><>]}><[<()()>[{}{}]](((){})<()>>>>}]<{({{([]()){{}<>}}<{<>{}}({}{}
{[{[[<[((<(<<{<>}{()[]}>><{<<>[]>[()[]]}{{()()}[()[]]}>)(({[[]](<>[])}{({}())<{}()>})([[{}[]]<[]()>][{
[{[(({[{{([[[<()()>{{}{}}]{([]<>){<><>}}]](<<<(){}>(()())>{[[]]{<>{}}>>))<[<((<>{}){(){}}){[{}<>]}>([[<><
<{<(<([(<[({<<[]()>[[]{}]>(<{}<>>)}(<[{}[]][{}[]]>{<{}[]>[(){}]}))[<(<[]<>>{{}<>}]([[][]]<()<>>)>{({{}<>}[<>[
{({({{<({[{[{<<>()>[[]{}]}](([{}<>]{()()})[([]()){[]()}})}[[((()())[()<>])(<<>{}><{}>)]]]<([{([]
(<[([{({<([<(([]<>)<()[]>){{<>{}}[()()]}>])(([({[]()}<{}{}>)][<{{}[]}({})>[[<><>][<><>]]])<[({()[]}[[]{
{{{<{{[[{[[<({[]()}{()[]})<[<>()][()()]>>(({{}{}}))][({[(){}](()[]]}){{<<>()>({})}{<{}[]>[{}<>]}}]]{[<
{<([((<{<<<{(({}{})<<>[]>)(((){}){[]})}{({[]<>}[[]{}])<[{}[]][()()]>}}[{([<>()][<>{}]){(<>
{([([(({[<[[[(())(<>[])]{[()()][<>]}]]>([{[(<><>)(<>())]<(()<>)[<><>]>}({{<>{}}({})}([{}][
([[[({[{{<{{{[{}()]{(){}}}[{()}({}[])]}[{[[]<>>(()[])}<{[][]}{[]}>]}>}}]}{[<[[[[(({}{}){{}[]}
<[{<([<({((([[<>{}]([]())]<[()[]]<()<>>>){<<{}{}>[<><>]>[((){}){{}<>}]}))}({[({[()())(<>[])}{({}())}){([()[]
{([[{[<<[[<{([<>()][{}{}])<([]{})(()[])>}{{{()()}{[][]}}<[[]{}]<<>{}>>}>(({<()()>({}())}[{<>[]}({}<>)])([[<
({[<{{[<{{(((([]<>){<>()})([()<>]([]{})))){{<{()()}[{}<>]>}({<{}[]>}{([]<>)<{}[]>})}}<<([{{}()}[{}[]]]<{(
<({{([{[({<([({}())[()[]]]{([]{})})(([[]{}]{{}[]})([{}()><()<>>))>(<(({}{})<{}{}>)[{<><>}{[]
[{[{<{([[[<[[{[][]}(<>{})]{{[]<>}[{}{}]}]><{({{}()}<[]()})}>]{{(<[[][]]{[]()}>{(()())(<>[])})(
(<{(<[[<{[[<<([]<>)<{}{}>>({{}[]}({}()))>]({([[]<>]([]())){[<>{}]<{}()>}}[{({}())[(){}]}(({}{})[[]]
<[<<([{[{<{{<[[][]]<<>[]>>({<>{}}[<><>])}([[{}<>]<()()>][<[][]>(()[])])}[({[()()][[]()]}[{{}()}<[]<>>])]>{<
<(([[{(([[([[{{}()}{[]}]]<((<><>}({}[]))((()())[()[]])>)[{{<[]<>>((){})}}<(<(){}>{{}[]})({()}{{}<>})>]]<[(<{
[{<(((<(<({(({{}()}{<><>})[{[][]}{(){}}])({{[]{}}[<>{}]>)})>((([<(<><>){{}<>}><[<>{}]>]([[<>[]][{}{}
[<{{({([({({{{()<>}([][])>}((({}<>)(()[]))<(<>()){{}<>}>)){[([()()](<>[]))]{({[]}[[]{}])([[]<>
{({[<{(<<([[[<{}{}>[{}{}]](<<>>(()))]{[([]<>)<{}{}>]<[<><>]>}]<[[{{}<>}<<>{}>][([]<>)<(){}
([{{{[[(<<[<[[<>]{[]}]((<>{})({}[]))>({{()[]}<<><>>}{{<>{}}({})})]{{{{[][]}<{}()>}}<[[<><>][{}<>]]<
[<{<{<[{{[([(([]<>))[{<>()}<{}{}>]]{[<<>{}>[(){}]){<[][]>[<><>]}})](([<([]{})[{}()]>(({}())(<>{}))]
[<[<{(<(([{<{([]{}){[][]}}><{[{}[]]([]<>)}[[()[]]<()()>]>}][{(<[(){}]{()<>}><<[]()>[[]()]>){[[<>[]](()[])
<(<<<([[<([[({[]<>}<[]()>){{<>[]}[{}{}]}]([[[][]]<[]()>])]<{{(<><>){<>[]}}}{(<[][])(()[]))[([]{})[[
(([{<{{([<<{({[]{}}[()[]])[<()()><()[]>]}[(<()>[()()])[[<>[]]]]>>{{{{<[]()>[()<>]}}{{<()[]>[()[]]}<[[][]]
(<<[([[{{({<[([][])<{}()]][<()()>(()())]>})}<([[{(()<>)<[]()>}((()[])[<>{}])]]([<{(){}}({}<>)>][<
[<<((<[<<{(((({}()))({<>{}}([]<>))){{<()<>><{}[]>}{(()())}}){({([]<>)<<>{}>})}}<{{<[()][<><>]>{{[]
<{[<[((([{<{{{<>[]}[[]()]}([<><>])}>((<<<>()>({}<>)><[{}[]]<()[]>>)[{{[][]}{()<>}}[{()<>}(()[])]])}(([<{{
{<{<[(({([<[<[[][])([]{})>[{(){}}[[]<>]]][(<{}<>>{()<>}){[<><>]{{}[]}}]>([[<[][]>{[]<>}]({[][]}(()()))]{
(<{{{[{<{(<[({{}[]})]><<{(()<>)[{}{}]}{<()()>}>>)[[(({()<>}(<>{})))([<(){}>(()<>)]{((){})([][])})]
<<[(<([<(({[{({}{})}<{()<>}{<>{}}>][([[]()](()<>))]}[[({{}()}<[]{}>)[<[]()>[()()]]]((<{}{}
<<{[{[<{{{([<<<>{}>{()[]}>({{}[]}<<>{}>)][[<()[]>[{}[]]]({()<>}(()()))])[<<<<>()>{[]{}}>><{{[]{}}<()<>>}(<<>[
[{{<({[([({{{(()())[<><>]}{[()()][(){}]}}}{[([<>[]])[{[]()}((){})]]([<[]{}>(()<>)]{[<>]})})(<
<<<{{(({<{<[<<<>{}>>(<<>[]>{()()})]<([()<>])>>[(<({}[]){{}[]}>{[{}{}][[]()]})[<[<>[]]<()[]>>(<
[([<<<(<[[[[[([]{}){{}()}}<<[]()>{[]}>][([<>[]]({}))[(()<>){()()}]]]]]>{[([[<([]())<{}<>>>({<>()}
[([{{[[(({[<{[{}[]]{[][]]}<{<>()}[<>[]]>>(<{<>()}<[]()>>)]}))<<<<<{{()[]}{{}[]}}[<<>{}>]><{[{}<>]{(
[{{{<((([{([{[[]()][<>()]}([[]()]{{}()})]{([{}{}](<>{}))({()<>}(<>{}))})<[({(){}}[()<>])}>}(
{<<{{({[[[[[{[()]({})}<[<><>]{{}}}][[{()<>}({}{})][<<>[]>{<><>}]]]({<(<>)<<><>>>({<>()}<<>
<[[[[({([[{<[{{}()}]{(()<>)<[]()>}>{[{<>{}}(()[])][{()[]}<()()>]}}]]<{[(({<>{}}{<>()})<[[]()]<<>{}>>)<((()[])
({{{<{[({[[[<{()<>}<<>[]>>{[[]<>][<>()]}]([(()<>)[[]()]]({{}[]}(()[]))]]][{{{<<>>({}<>)}<<<>{}
[{[((<{<([[{<<()[]>[[][]]>}<{({})}[<{}[]>[[][]}]>]]<<{(([]<>)[[]()])}<{<{}()>{[]}}{[()[]]({})}>>
[{([[{[({<[<(<<>{}>({}[]))([{}<>]{(){}})}({<[][]>}{[()()]<{}[]>})]<<[<(){}>[<>{}]][<<>{}>({}{
[<([{<<[((([<[<>()]({}())><([]{})<{}[]>>]))[{[[[[]()]]]}{{[{[]()}([][])]{<[][]>{{}()}}}}])<(({{(<>()){
{{[[<{{(({{{(<{}<>>{()[]})[<[]()>]}[{<[]{}>[()[]]}]}<[{[[]<>][[][]]}<<(){}>((){})>][([()<>]<<>>)<
<<{[(([{<((({([][]){[]<>}}{({}())}))({[<<>{}><<>>]}{<[<>{}](()())><<{}[]>{()<>}>)))({[<(<>[
<[<((<<[[<[((([]<>)<(){}>)[<(){}>])({<{}{}>{<><>}}<(()())<[]{}>>)]{{({(){}}(()[]))}<({<><>}({}()))>}
[<{(<([{[({[{({}{})<{}()>}[{[]<>}({}<>)]]})[<[([{}[]][()[]])((<>()){{}<>})][[[(){}][[][]]][<
<(({([[(<[[{(<<><>><{}()>)<(()()){()[]}>}({[()<>][{}[]]}([<>{}]<<><>>)}]]{{[<[<>[]]>({<><>}([]{}))][(([][])<
{[[[[<<[{(({{{[]()}[{}[]]}}{[[()()][{}{}]]}){[{{{}()}}[[[][]]<()()>]]}){[{((()<>)[{}<>]){<<>()>[()()]}}(([{}<
[[<<<{<({[<([[()<>]]({()<>})){[{{}[]}]}>][({[<()<>>({}[])]}((<<><>>[<>])[({}())<{}{}>]))]})>}>[{{(({{[
{{[[{{(([[[[({[]()}<{}()>)<(()[])<<>[]>>]]<(<{[][]}<{}()>>(({})[[][]]))>]<<{({()[]}[(){}]]({
({{((<([<({<(<[]<>><<>[]>){[{}<>]}>(<[{}{}]>)}[<{{<>{}}{[]()}}[<<><>>[[][]]]>[[<()()><(){}>]{<
[[[(({{<(<{<[{()[]}]({<>{}})>[<<<>()>{{}{}}>{({}<>)<<>()>}]}<({{()()}}<(()()){<><>}>){<((){})(<>()
{<{{(<{{<[[({<<>>}<[(){}]({}())>)[<{()<>}[{}()]>[({}[])<()()>]]]]{<[({<>{}}{[][]})]{[(<>{})({}())]}
[<[<{<<({[[(((<>())[{}<>]))<(({}[])(()[])){{{}{}}[[]<>]}>]<[(<{}{}]{{}{}})<(()<>)>][[{<>{}}<[
[({<(<[([<({([<>()]{[][]}>{<<>[]>[[]()]}}[{<()<>>([]<>)}])<<<[[]()](())>><[<<>[]>{{}()}]{<()
{{[{(<<{[{({[<[]()>{{}<>}]([()<>]{{}[]})}<<<[]()>[[]<>]><[<>{}][()()]>>)}<{<[{[]()}{<>{}}]>{({[][]}<[]
[[{[<<<[{[[<({<>()}((){})){{<>[]}<(){}>}>[<(()<>)({}<>)>{<[]()>(<>)}]]]{<<{[<>[]]>>(<<{}()>(<>())>(
{({{[([<{{{<((<><>)<()()>][<{}[]>{<>[]}]><({<>()}(()<>))({{}[]}<()[]>)>}([({{}[]})({{}()}[<>[]])])}
{<[[({{((<[{[{<>()}(<>())]<({}[])<<>{}>>}([(()())<<>[]>]((<>[]){{}{}}))]{({<()()>(()<>)}[{[][]}([]())])<<[()
([{((({<<<{<{{<><>}(()<>)}>}>)[{{[[[[][]]]][<(<>{}){[]}>([{}()][[][]])]}<{([()()][<>()])(<[]()>
<(((<(({<<[{<(()())>[[[][]]<[]{}>]}[{{<>()}(<><>)}(({}())({}()))]]>((<({{}[]}(<>())){({}[])
<<{[(<([<(<{{[<>{}]}(<<>()>({}{}))}><({<<>()>(<><>)}<<[][]>[[]()]>)(([<>()){{}()})<[[][]]{<>{}
<<[[([{{{{[<{[(){}]<[]()>}>[[([])<[]<>>>[({}{})([][])]]]}<<({<()[]><{}<>>}<{()<>}(<><>)>)>(([
{[[[{{[{{((<{[{}[]](()[])}<(<>{})(<><>)>>){(<<<>{}>>)(([()[]]<[]{}})[({}[])(()())])})}}]}{[{{({({([]
([{[{((([[{{([[][]][{}[]])[{[]<>}<<>[]>]}}]][<[{[(<>{})[[]<>]][{[]<>}[(){}]]}]{<[[{}[]]{{}[]}]><(({}
{(<(<{<<([<<<[{}{}]<{}{}>>><[{<><>}{{}<>}]([[]<>]{{}()})>>])>>{{[{{{<<<>()>[{}()]>{<[]()>[<>{}]}}}}<(<[(
{<[{<<({[({(([[]<>](<><>)))[[[{}()]<()[]>]<{<>{}}[()<>]>]})([<([(){}]{<>[]})<<<>()><(){}>>><([<>[]>([][]))(<<
[[[[<<(<({(<[{(){}}[()[]]][[{}[]]{[]{}}]>([{[][]}([]{})][{{}[]}<[]()>]))[{[{()()}({})]({(){}}[<>[]
{<{(<[<(<<<{<[()<>]>{{()[]}}}<<([]<>)[()<>]>>>[{[<<><>>[{}<>]][<<><>>{(){}}]}[{[[]()]{<>()}}
<[[<{<<[{{[[(([][])[[][]])<({}<>){{}[]}>]](<<({}[])[[][]]>((<><>))>[([()[]]((){}))])}}{[([((<>())
{(<{[[((<<<{<{<>{}}([]{})><{()}>}[{(()<>)[[][]]}{<{}{}><{}{}>}]]<[[<{}()>]<([]{})>]{{[<>()]{{}{
<{<{<{[{(((<(<[]{}>)([{}{}]({}{}))>([{[]<>}[[]{}]]<([]()){[]<>}>))<[(<[]{}>[()[]])]>)<{{[{{
{{<([<{[(([(({[]<>}{()[]})<(()){(){}}>)[{({}<>)<[]<>>}<{{}()}<<><>>>>][{<({}<>)[()()]><<()<>>>
[[[<([<<[[[{<<[][]>{()()}>[[(){}][<><>]]}{[{<>[]}[[]{}]]}]{<<({}<>)[[]]>[(<>[])]><<[()[]]<[]>>[{[]<>}({}{})]>
{{{[{(<<{({{<{<>[]}>[{<><>}[()[]]]}[({()()})([<><>]<<>[]>)]}({[{{}[]}{{}[]}]}[(<[]()>[<>[]])({<><>}[(){}])]
(<<[{[{[({<[{{(){}}<(){}>}{{()[]}[<>{}]}]{{(<>{})<{}>}{((){})}}>[{([<>()](<>{}))<{{}[]}[[]{}]>}{(<[][
(<[[{<<{({<([<<>()>[<>()]]{{()<>}[<>[]]))>({<<<>()><()<>>>}({{[]<>}{{}()}}{<(){}><<>{}>}))})}<([<({[[]]{{}<
([((<[((([{<((()<>))[([])]>({(()<>)([]{})}([{}<>](()<>)))}{<[[[]()]([]{})]>}]({([{[]{}}][{(){}}{(){}
(({{<[<{<{([<([]<>){()}><([]<>)<[][]>>]<<[()<>][[]<>]><{()()}>>)[(<[[]{}](<>())>{{{}()}{[]()}}
";
