---
Kao i svaki distribuirani sistem, FastComments treba način za zaključavanje resursa i procedura. Ova zaključavanja se mogu nadgledati putem endpointa `/locks-in-progress`.

[Na primer, evo endpointa na našem US shardu](https://fastcomments.com/locks-in-progress).

Ovo može biti korisno da se vidi zašto je sistem zablokiran ili pod velikim opterećenjem. Ako, na primer, SRE želi da vidi zašto sistem ima visok CPU load, mogli bi
proveriti ovaj endpoint da dobiju ime cron zadatka koji se nepravilno ponaša.

---