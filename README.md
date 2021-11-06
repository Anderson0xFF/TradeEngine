# TradeEngine


Introdução

Um "mecanismo de negociação" é responsável por processar as ordens que os usuários criam
em um exchange. 

O Trading engine possui um componente principal denominado livro de
ordens (orderbook). Ele se encarrega de acumular as ordens até que sejam correspondidas ou
canceladas pelo usuário.

Um orderbook classifica os pedidos em dois grupos: pedidos de venda e pedidos de compra.
Os pedidos de venda são sempre acessados do mais barato ao mais caro e os pedidos de
compra do mais caro ao mais barato.

Os pedidos podem ser de compra ou venda.
Cada vez que um novo pedido é inserido, ele é avaliado em relação ao lado oposto da carteira
de pedidos, se houver uma chance de corresponder.

## Implementações
 - [x] OrderBook
 - [x] Processamento de orders completas ou parciais.
 - [x] Modulo de Testes
 - [x] Logs em Json

## Compilando 
~~~shell
cargo build
cargo run
~~~
