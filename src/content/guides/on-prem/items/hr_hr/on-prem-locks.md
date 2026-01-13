---
Kao i svaki distribuirani sustav, FastComments treba neki način za zaključavanje resursa i procedura. Ta zaključavanja se mogu pratiti putem `/locks-in-progress` endpointa.

[Na primjer, evo endpointa na našem US shardu](https://fastcomments.com/locks-in-progress).

Ovo može biti korisno za utvrđivanje zašto je sustav zapeo ili pod opterećenjem. Ako, primjerice, SRE želi vidjeti zašto sustav ima visoko opterećenje CPU-a, mogli bi
provjeriti ovaj endpoint kako bi dobili ime crona koji se ne ponaša ispravno.

---