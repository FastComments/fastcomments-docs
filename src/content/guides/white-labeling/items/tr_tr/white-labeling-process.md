---
White Labeling'i kurma süreci aşağıdaki gibidir:

1. Faturalamanın nasıl yönetileceğine karar verin.
   1. FastComments Pro ile, belirli sayıda beyaz etiketli kiracı için sabit bir ücret ödersiniz.
   2. FastComments Flex ile, her kiracı ve o kiracının kullanımı için ödeme yaparsınız.
   3. Her iki durumda da her kiracının limitlerini siz belirlersiniz.
      1. Limitler kiracı bazında özelleştirilebilir. Ayrıca, sattığınız paketleri güncellerseniz, mevcut müşterilere zaten sağladığınız fiyatlandırmayı bozmadan bunu yapabilirsiniz.
2. FastComments.com terminolojisine aşina olun:
   1. `Tenant` bir "müşteridir".
   2. `TenantUser` `Tenant` içinde belirli ayrıcalıklara sahip bir yöneticidir.
   3. `TenantPackage` bir `Tenant` için mevcut olan belirlenmiş limitler ve fiyatlandırma içeren bir pakettir.
3. Kiracıları sisteme almak için [API](/guide-api.html) ile entegre olun veya [scripts](https://github.com/FastComments/fastcomments-code-examples/tree/master/white-labeling) kullanın.

---