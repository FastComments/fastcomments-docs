In FastComments'ta makale kimliğine veya yorumların bağlandığı sayfaya, bir URL veya ID olabileceği için URL ID diyoruz.
URL ID'yi aşağıdaki şekilde tanımlayın. Bileşen config nesnesindeki değişiklikleri izler ve yeniden yükleme yapar, bu yüzden sadece "url" ve "urlId" ayarlarını güncellemeniz yeterlidir.

Tam çalışan bir örneği [burada](https://github.com/FastComments/fastcomments-vue/blob/master/dev/serve-pagination.ts) görebilirsiniz.

Sayfalama örneğini şu komutla çalıştırın:

```
npm run serve-pagination
```

```vue
<fast-comments-vue v-bind:config="{tenantId: 'demo', url: 'https://example.com', urlId: 'some-page-id'}" />
```

### Hesap Bölgesi (DİKKAT: AB Müşterileri)

Hesabınız AB'de bulunuyorsa, widget yapılandırmasında `region = 'eu'` olarak ayarlayın, örneğin:

```vue
<fast-comments-vue v-bind:config="{tenantId: 'demo', url: 'https://example.com', urlId: 'some-page-id', region: 'eu'}" />
```

Aksi halde `region`'ı tanımlamanıza gerek yoktur.