[related-parameter-start name = 'countAll'; type = 'boolean'; related-parameter-end]

Броят на коментарите, показван в горната част на уиджета за коментари, може да показва или всички "top-level" коментари, което означава тези отговори, които са отговори директно към страницата или самата статия, или може да бъде брой на **всички** вложени коментари.

По подразбиране това е `true` - това е брой на последното - всички коментари. В по-старите версии на уиджета за коментари стойността по подразбиране е `false`.

Можем да променим поведението, така че то да бъде брой на **всички** вложени коментари, като зададем флага **countAll** на true.

[code-example-start config = {countAll: true}; linesToHighlight = [6]; title = 'Counting All Comments'; code-example-end]

Ако искаме броят да отразява само коментарите от най-горно ниво, задаваме флага на false.

[code-example-start config = {countAll: false}; linesToHighlight = [6]; title = 'Counting Top Level Comments'; code-example-end]

В момента това не може да бъде персонализирано без промени в кода.