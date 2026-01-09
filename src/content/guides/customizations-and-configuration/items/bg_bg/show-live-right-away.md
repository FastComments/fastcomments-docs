[related-parameter-start name = 'showLiveRightAway'; type = 'boolean'; related-parameter-end]

По подразбиране активността в секцията за коментари в реално време е включена. Това означава, че ако се добавят, изтриват, редактират или прикрепят коментари, промените трябва да се появят
за всички потребители, които преглеждат нишката с коментари по едно и също време.

Въпреки това, по подразбиране тези нови коментари ще се появят под динамично показан бутон с текст, подобен на "Show 2 New Comments".

Ако новите коментари са отговори директно на страницата, бутонът ще се покаже в горната част на нишката с коментари. Ако са отговори на конкретен коментар, 
бутонът ще се покаже под този коментар.

Това е, за да се предотврати постоянното променяне на размера на страницата за потребителя, което може да причинява раздразнение при опит да хване лентата за превъртане.

За някои случаи на използване, като живо наддаване или онлайн събития, това не е желаното поведение - може да искате уиджитът за коментари да бъде
по-скоро като чат прозорец, където новите коментари "show right away".

Hence, the name of the flag that enables that feature: **showLiveRightAway**.

We can turn it on as follows:

[code-example-start config = {showLiveRightAway: true}; linesToHighlight = [6]; title = 'Show Live Comments Right Away'; code-example-end]

This can be customized without code, on the widget customization page:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.collapse-live-comments'; selector = '.collapse-live-comments'; title='Show Live Comments Right Away' app-screenshot-end]

---