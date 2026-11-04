Визначення того, як взаємодіють кілька користувачів, і її тестування — це складне завдання. Ось специфікація, якої ми дотримуємося для контролю доступу,
і яку ви можете використати для тестування своєї реалізації:

    Сторінка з null group ids, користувач з null group ids - доступ має бути.
    Сторінка з null group ids, користувач з group ids - доступ має бути.
    Сторінка з group ids, користувач з null group ids - доступ має бути.
    Сторінка з group ids, користувач з порожнім списком - доступу НЕ має бути.
    Сторінка з group ids, користувач з group ids - перетин існує - доступ має бути.
    Сторінка з group ids, користувач з group ids - перетин відсутній - доступу НЕ має бути.
    Сторінка з порожнім списком group ids (доступ ні в кого), користувач з null - доступу НЕ має бути.
    
    SSO User A = Group ids не визначені (null = повний доступ).
    SSO User B = Group ids не визначені (null = повний доступ).
    A може @B.
    
    SSO User A = Group ids не визначені (null = повний доступ).
    SSO User B = Group ids визначені.
    A може @B.
    
    SSO User A = Group ids визначені.
    SSO User B = Group ids не визначені (null = повний доступ).
    A може @B.
    
    SSO User A = Group ids = [a].
    SSO User B = Group ids = [b].
    A НЕ може @B.
    
    SSO User A = Group ids = [a].
    SSO User B = Group ids = [a, b].
    A може @B.