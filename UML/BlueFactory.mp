input metauml;
beginfig(1);
    iClass.foreColor := (0.7, 0.7, 1);
    iClassNameOnly.foreColor := (0.7, 0.7, 1);

    Class.BlueFactory("BlueFactory")
    ()
    ("+ CreateButton()",
    "+ CreateScroller()",
    "+ CreateProductN()");

    ClassName.BlueButton("BlueButton");
    ClassName.BlueScroller("BlueScroller");
    ClassName.BlueProductN("BlueProductN");
    
    classStereotypes.BlueFactory("<<AbstractFactory>>");

    classStereotypes.BlueButton("<<Button>>");
    classStereotypes.BlueScroller("<<Scroller>>");
    classStereotypes.BlueProductN("<<ProductN>>");

    leftToRight.midy(30)(BlueFactory, BlueScroller);
    topToBottom.left(10)(BlueButton, BlueScroller, BlueProductN);

    drawObjects(BlueFactory, BlueButton, BlueScroller, BlueProductN);

    clink(dependency)(BlueFactory.methodStack.pict[0], BlueButton);
    clink(dependency)(BlueFactory.methodStack.pict[1], BlueScroller);
    clink(dependency)(BlueFactory.methodStack.pict[2], BlueProductN);
endfig;
end