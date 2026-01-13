Тепер ми згенеруємо ваш власний код FastComments. Використайте майстер нижче, щоб налаштувати, як ви хочете, щоб FastComments працював на вашому сайті GoHighLevel:

[snippet id="gohighlevel-wizard"]

### Різні типи блоків коментарів

Ви можете налаштувати рядок `TYPE = 'commenting'`, щоб перемикати використовуваний продукт (наприклад, ви можете змінити його на `live` для стрімінгового чату або `collab` для collab chat).

### Розміщення блока коментарів там, де ви хочете

Припустимо, ви хочете розмістити блоки коментарів у конкретних частинах сторінки, а не в стандартних місцях.
Змініть цей рядок:

    const TARGET_ELEMENT_ID = ''; // set to use target div mode

На:

    const TARGET_ELEMENT_ID = 'fc_box'; // set to use target div mode

Потім у редакторі GHL натисніть кнопку "code" і додайте туди, куди ви хочете, щоб з'являлися коментарі:

[inline-code-attrs-start title = 'FastComments Div для GoHighLevel'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<div
  id="fc_box"
  type="commenting"
  urlid="custom-chat-id"
></div>
[inline-code-end]

### Різний тип блока коментарів для кожної сторінки

Припустимо, ви хочете, щоб користувачі виділяли й обговорювали фрагменти тексту, або натомість використовували інтерфейс стрімінгового чату.

Спочатку виконайте кроки вище в "Розміщення блока коментарів там, де ви хочете".

Зверніть увагу, що в тому невеликому фрагменті є `type="commenting"`.

Наприклад, якщо ви хочете увімкнути collab chat, змініть type на `type="collab"`.

### Показувати лише на конкретних сторінках

Якщо ви не встановите не встановите `TARGET_ELEMENT_ID`, ви можете натомість налаштувати змінну `VALID_PATTERNS`, щоб визначити, на яких маршрутах URL повинні відображатися коментарі. За замовчуванням вони будуть показані на сторінках, які містять `/post` у URL.

### Налаштування collab chat

Ви можете вказати collab chat додавати спільні функції лише навколо HTML всередині певної області. Наприклад, припустимо, ви додали код футера вище, а потім додаєте цей div у вміст запису/сторінки, щоб увімкнути collab chat:

[inline-code-attrs-start title = 'Collab чат із вказаним вмістом'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<div
  id="fc_box"
  type="collab"
  urlid="custom-chat-id"
><p>This content will have collab chat!</p></div>
[inline-code-end]

Тоді елемент параграфа всередині `<div>` матиме увімкнений collab chat, і нічого іншого на сторінці. Якщо ви не помістите жодного вмісту в `<div>`, тоді collab chat буде увімкнено в усьому тілі запису.