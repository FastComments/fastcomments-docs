---
Her dağıtık sistemde olduğu gibi FastComments'in kaynakları ve prosedürleri kilitlemek için bir yönteme ihtiyacı vardır. Bu kilitler `/locks-in-progress` uç noktası üzerinden izlenebilir.

[Örneğin, işte ABD shard'ımızdaki uç nokta](https://fastcomments.com/locks-in-progress).

Bu, sistemin takılmasının veya yüksek yük altında olmasının nedenini görmek için faydalı olabilir. Eğer bir SRE, sistemin yüksek CPU yükü yaşamasının nedenini görmek isterse,
bu uç noktayı kontrol ederek hatalı davranan cron'un adını öğrenebilir.

---