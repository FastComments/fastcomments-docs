---
Blackboard Learn SaaS и Ultra подржавају LTI 1.3 Dynamic Registration.

#### Отворите екран провајдера алата

1. Пријавите се у Blackboard као системски администратор.
2. Идите у **Administrator Panel** > **Integrations** > **LTI Tool Providers**.
3. Кликните **Register LTI 1.3 / LTI Advantage Tool**.

If you only see "Register LTI 1.1 Provider", ваша верзија Blackboard-а још не подржава LTI 1.3 - надоградите или контактирајте Blackboard support.

#### Залепите URL

Залепите FastComments registration URL у поље **Client ID** / **Registration URL** (називи поља у Blackboard-у варирају по верзијама). Потврдите.

Blackboard изводи registration handshake са FastComments и приказује екран за потврду.

#### Одобрите и омогућите

Blackboard означава ново-регистроване алате као **Approved but excluded** по подразумеваној поставци:

1. Пронађите унос FastComments у листи провајдера алата.
2. Отворите мени и изаберите **Edit**.
3. Подесите **Tool Status** на **Approved**.
4. У оквиру **Institution Policies**, проверите који се кориснички подаци шаљу (име, e-пошта, улога). Сачувајте.

Алат је сада доступан инструкторима када додају садржај у курсеве.

---