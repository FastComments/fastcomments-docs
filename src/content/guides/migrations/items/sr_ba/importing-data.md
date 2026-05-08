Iako FastComments Support може помоћи при миграцијама, већина се може обавити и пратити лако без икакве интервенције
особља за подршку.

Нативно подржавамо увоз извозних фајлова од следећих провајдера:

- Commento
- Disqus
- Hyvor Talk
- Muut Comments
- IntenseDebate
- Just-Comments
- WordPress (via the plugin)
- AnyComment (Via WordPress Import/Export)

Идући на [овде](https://fastcomments.com/auth/my-account/manage-data/import) можемо отпремити фајл који садржи податке за миграцију.

[app-screenshot-start url='/auth/my-account/manage-data/import'; selector = '.account-block'; title='The Import Page Form' app-screenshot-end]

### Praćenje uvoza

FastComments користи систем за обраду задатака за процесирање увоза и извоза. Када систем преузме ваш задатак, он ће
повремено пријављивати статус задатка у интерфејсу за увоз или извоз.

[app-screenshot-start url='/auth/my-account/manage-data/import?demo=true'; selector = '.content'; title='Import Job Status' app-screenshot-end]

Имајте на уму да су статуси увоза и извоза видљиви свим администраторима на налогу.

Ако ваш задатак не успије, он се неће аутоматски поново покренути. Увоз ће морати бити покушан поново. Ако било који увоз или извоз не успије,
наши системски администратори ће бити аутоматски обавијештени. Ако откријемо проблем, контактираћемо вас да видимо можемо ли помоћи.

### Ponovno pokretanje uvoza

Током неких миграција, неопходно је покретати увоз више пута. На примјер, уобичајено је урадити прву миграцију
ради тестирања, а затим поново покренути увоз са најновијим подацима прије коначног пребацивања.

Поновни увоз истог садржаја **неће креирати дупликате**.

### Sigurnost podataka i isteka

Фајлови за увоз нису доступни путем вањских захтјева на било који начин, а фајлови за увоз се бришу из нашег система чим
се увоз заврши.

---