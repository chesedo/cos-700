input metauml;

beginfig(1);
    Interface.AbstractVisitor("Visitor")
        ("+Visit_Button(e: Button)",
        "+Visit_Scroller(e: Scroller)");
    classStereotypes.AbstractVisitor("<<interface>>");
    AbstractVisitor.info.foreColor := white;

    Class.DrawVisitor("DrawVisitor")()
        ("+Visit_Button(e: Button)",
        "+Visit_Scroller(e: Scroller)");

    Class.NameVisitor("NameVisitor")("List<String>: names")
        ("+Visit_Button(e: Button)",
        "+Visit_Scroller(e: Scroller)");

    centered_allign_top(DrawVisitor, NameVisitor)(20, below(AbstractVisitor.s, 25));

    drawObjects(AbstractVisitor, DrawVisitor, NameVisitor);

    link(inheritance)(pathStepY(DrawVisitor.n + (10, 0), AbstractVisitor.s, 5));
    link(inheritance)(pathStepY(NameVisitor.n, AbstractVisitor.s, 5));
endfig;

end