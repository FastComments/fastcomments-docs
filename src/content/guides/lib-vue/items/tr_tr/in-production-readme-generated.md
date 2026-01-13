Muhtemelen callback'ler vb. geçiriyorsanız config'i satır içi olarak tanımlamak istemezsiniz. Bunun yerine
config'i bir `computed` bloğunda tanımlamak istersiniz; aksi takdirde her seferinde callback'iniz çağrıldığında tüm widget yeniden render edilecektir.

[Bunu nasıl yapacağınızı görmek için spinner örneğine bakın.](https://github.com/FastComments/fastcomments-vue/blob/master/dev/serve-spinner.vue)