struct MyStruct {
    field1: i32,
    field2: i32,
}

impl MyStruct {

    fn bar(&mut self) -> &mut i32 {
        self.field1 = 1;
        self.field2 = 2;
        &mut self.field1
    }

    fn foo(&mut self) -> i32 {

        let f1 = 
            self.bar();  // mutable borrow occurs here

        /*
        Ошибка не из-за того, что вы «захватываете» field2 где-то заранее — дело в том, как именно вы получаете f1.

        Когда вы пишете self.bar(), вызывается метод с сигнатурой fn bar(&mut self) -> &mut i32. 
        Компилятор видит это как:

        - заимствование всего *self как &mut (а не только field1),
        - возвращаемая ссылка &mut i32 имеет lifetime, привязанный к этому заимствованию всего self.

        С точки зрения borrow checker'а, внутри bar возвращённая ссылка теоретически может указывать на любое поле — 
        компилятор не анализирует тело метода через границу вызова функции (нет "field-level" анализа сквозь функции). 
        Поэтому пока жив f1, весь self считается замороженным под &mut, и let f2 = &self.field2; конфликтует с этим заимствованием.
        */

        let f2 = &self.field2;  // immutable borrow occurs here
        //  cannot borrow `self.field2` as immutable because it is also borrowed as mutable

        /*
        Если бы вы написали прямо в foo:

        let f1 = &mut self.field1;
        let f2 = &self.field2;

        то это сработало бы — компилятор видит конкретные разные поля и применяет disjoint field borrows (split borrows), 
        которые работают только при прямом доступе к полям, написанном "на виду", а не через вызов метода.
        */

        *f1  // mutable borrow later used here
            + *f2

    }

}

fn main() {
    let mut my = MyStruct {
        field1: 1,
        field2: 2,
    };
    println!("{}", my.foo());
}
