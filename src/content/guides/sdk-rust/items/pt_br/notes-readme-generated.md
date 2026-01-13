### IDs de Transmissão

Você verá que deve passar um `broadcastId` em algumas chamadas de API. Quando você receber eventos, receberá esse ID de volta, para que saiba ignorar o evento se planejar aplicar alterações de forma otimista no cliente
(o que você provavelmente desejará fazer, já que oferece a melhor experiência). Passe um UUID aqui. O ID deve ser suficientemente único para não ocorrer duas vezes em uma sessão do navegador.