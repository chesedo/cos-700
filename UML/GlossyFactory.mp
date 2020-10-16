input metauml;
beginfig(1);
    iClass.foreColor := (1, 0.7, 0.7);
    iClassNameOnly.foreColor := (1, 0.7, 0.7);

    Class.GlossyFactory("GlossyFactory")
    ()
    ("+ CreateButton()",
    "+ CreateScroller()",
    "+ CreateProductN()");

    ClassName.GlossyButton("GlossyButton");
    ClassName.GlossyScroller("GlossyScroller");
    ClassName.GlossyProductN("GlossyProductN");
    
    classStereotypes.GlossyFactory("<<AbstractFactory>>");

    classStereotypes.GlossyButton("<<Button>>");
    classStereotypes.GlossyScroller("<<Scroller>>");
    classStereotypes.GlossyProductN("<<ProductN>>");

    leftToRight.midy(30)(GlossyFactory, GlossyScroller);
    topToBottom.left(10)(GlossyButton, GlossyScroller, GlossyProductN);

    drawObjects(GlossyFactory, GlossyButton, GlossyScroller, GlossyProductN);

    clink(dependency)(GlossyFactory.methodStack.pict[0], GlossyButton);
    clink(dependency)(GlossyFactory.methodStack.pict[1], GlossyScroller);
    clink(dependency)(GlossyFactory.methodStack.pict[2], GlossyProductN);
endfig;
end