---
Her widget ve akışın, genel `demo` kiracısı üzerinde yerel olarak çalıştığını görmek için, depoyu klonlayın ve çalıştırın:

```bash
npm install
npm run build
cd examples/example-showcase
npm install
npm run dev
```

Örnekler, kütüphaneye `file:../..` üzerinden bağlanır, bu yüzden kök `npm run build` adımı `dist/` üretmek için bir kez gereklidir.

Her widget/akış, `examples/example-showcase/src/views/` altında kendi görünümüne sahiptir ve bunu doğrudan kendi React uygulamanıza kopyalayabilirsiniz.
---