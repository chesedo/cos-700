input metauml;

beginfig(1);
    Interface.AbstractVisitor("Visitor")
        ("+Visit_Button(e: Button)",
        "+Visit_Scroller(e: Scroller)");

    Class.Drawer("Drawer")()
        ("+Visit_Button(e: Button)",
        "+Visit_Scroller(e: Scroller)");

    Class.NameVisitor("NameVisitor")("List<String>: names")
        ("+Visit_Button(e: Button)",
        "+Visit_Scroller(e: Scroller)");

    centered_allign_top(Drawer, NameVisitor)(20, below(AbstractVisitor.s, 50));

    drawObjects(AbstractVisitor, Drawer, NameVisitor);

    link(inheritance)(pathStepY(Drawer.n + (10, 0), AbstractVisitor.s, 20));
    link(inheritance)(pathStepY(NameVisitor.n, AbstractVisitor.s, 20));
endfig;

end