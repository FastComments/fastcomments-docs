---
Дефинирането как множество потребители взаимодействат и тестването на това е усложнено. Ето следната спецификация, която следваме за контрол на достъпа, и която можете да използвате, за да тествате вашата имплементация:

    Страница с null group ids, потребител с null group ids - трябва да има достъп.
    Страница с null group ids, потребител с group ids - трябва да има достъп.
    Страница с group ids, потребител с null group ids - трябва да има достъп.
    Страница с group ids, потребител с empty list - трябва да НЯМА достъп.
    Страница с group ids, потребител с group ids - има пресечение - трябва да има достъп.
    Страница с group ids, потребител с group ids - няма пресечение - трябва да НЯМА достъп.
    Страница с empty list of group ids (никой няма достъп), потребител с null - трябва да НЯМА достъп.
    
    SSO User A = Няма дефинирани group ids (null = пълен достъп).
    SSO User B = Няма дефинирани group ids (null = пълен достъп).
    A може да @B.
    
    SSO User A = Няма дефинирани group ids (null = пълен достъп).
    SSO User B = Има дефинирани group ids.
    A може да @B.
    
    SSO User A = Има дефинирани group ids.
    SSO User B = Няма дефинирани group ids (null = пълен достъп).
    A може да @B.
    
    SSO User A = Group ids = [a].
    SSO User B = Group ids = [b].
    A НЕ може да @B.
    
    SSO User A = Group ids = [a].
    SSO User B = Group ids = [a, b].
    A може да @B.

---