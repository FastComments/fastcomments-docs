FastComments'te yorumların bağlandığı makale kimliğine veya sayfaya URL ID diyoruz çünkü bir URL veya bir ID olabilir.
URL ID'yi aşağıdaki şekilde tanımlayın. Bileşen config nesnesindeki değişiklikleri izler ve yeniden yüklenir, böylece URL ID'yi güncelleyebilirsiniz.

```vue
<FastComments v-bind:config="{tenantId: 'demo', url: 'https://example.com/somepage', urlId: 'some-page-id'}" />
```

### Hesap Bölgesi (DİKKAT: AB Müşterileri)

Hesabınız AB'de bulunuyorsa, widget yapılandırmasında `region = 'eu'` olarak ayarlayın, örneğin:

```vue
<FastComments v-bind:config="{tenantId: 'demo', url: 'https://example.com/somepage', urlId: 'some-page-id', region: 'eu'}" />
```

Aksi takdirde `region`'ı tanımlamanıza gerek yoktur.