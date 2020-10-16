input metauml;
beginfig(1);
    Interface.Factory("AbstractFactory")
    ("+ CreateButton()",
    "+ CreateScroller()",
    "+ CreateProductN()");

    ClassInfoCopy.iProduct(iInterface);

    iProduct.iName.top := 10;
    iProduct.iName.bottom := 10;
    iProduct.iMethodStack.top := 0;
    iProduct.iMethodStack.bottom := 0;

    EClass.Button(iProduct)("Button")()();
    EClass.Scroller(iProduct)("Scroller")()();
    EClass.ProductN(iProduct)("ProductN")()();

    leftToRight.midy(30)(Factory, Scroller);
    topToBottom.left(15)(Button, Scroller, ProductN);

    drawObjects(Factory, Button, Scroller, ProductN);

    clink(dependency)(Factory.methodStack.pict[0], Button);
    clink(dependency)(Factory.methodStack.pict[1], Scroller);
    clink(dependency)(Factory.methodStack.pict[2], ProductN);
endfig;
end