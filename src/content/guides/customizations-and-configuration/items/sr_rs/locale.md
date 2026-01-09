---
[related-parameter-start name = 'locale'; type = 'string'; related-parameter-end]

По подразумеваној поставци, FastComments ће приказати видгет за коментаре у локалу који одређује систем и прегледач корисника.

Када корисник остави коментар или се пријави, ажурирамо њихов последњи коришћени локал и користимо га и за слање е-порука.

Ово утиче на начин на који је видгет за коментаре преведен за корисника. Локал се састоји од језика и региона корисника, па ће конфигурисање локала
обично променити језик који се користи за приказ текста кориснику.

#### Преко корисничког интерфејса

Ово се може дефинисати помоћу интерфејса за прилагођавање видгета. Погледајте опцију "Locale / Language":

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.locale-override'; title='Changing The Locale / Language' app-screenshot-end]

#### Помоћу кода

Ово може бити замењено жељеним локалом.

[code-example-start config = {locale: 'ru_ru'}; linesToHighlight = [6]; title = 'Manually Defining the User\'s Locale'; code-example-end]

### Подржани језици и кодови локала

[Овде можете пронаћи комплетну листу подржаних језика и одговарајућих кодова локала.](/guide-supported-languages.html#supported-languages)

### Напомена о SSO

Ако користите SSO, можда ћете желети да проследите локал корисника у user object, тако да су е-поруке и друге ствари правилно локализоване за њих.

---