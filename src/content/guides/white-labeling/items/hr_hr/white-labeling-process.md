---
Postupak za postavljanje White Labelinga je sljedeći:

1. Odaberite kako će se naplata obavljati.
   1. S FastComments Pro plaćate fiksni iznos za najviše određeni broj white-labeled tenant-a.
   2. S FastComments Flex plaćate za svakog tenant-a i za korištenje tog tenant-a.
   3. U oba slučaja postavljate ograničenja za svaki tenant.
      1. Ograničenja se mogu prilagoditi na razini pojedinog tenant-a. Dodatno, ako ažurirate pakete koje prodajete, možete to učiniti bez narušavanja cijena koje ste već dali postojećim korisnicima.
2. Upoznajte se s terminologijom FastComments.com:
   1. `Tenant` je "korisnik".
   2. `TenantUser` je administrator s nizom privilegija u `Tenant`.
   3. `TenantPackage` je paket s nizom ograničenja i cijena dostupnih `Tenant`u.
3. Integrirajte se s našim [API](/guide-api.html) ili koristite [skripte](https://github.com/FastComments/fastcomments-code-examples/tree/master/white-labeling) za uvođenje tenant-a.

---