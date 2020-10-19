input metauml;

beginfig(1);
    Interface.Element("Element")
              ("+Accept(v:Visitor)");

    Class.Button("Button")()
                    ("+Accept(v:Visitor)");
    Class.Scroller("Scroller")()
                  ("+Accept(v:Visitor)");

    Note.noteAcceptButton("v->Visit_Button(self)");
    Note.noteAcceptScroller("v->Visit_Scroller(self)");

    centered_allign_top(Button, Scroller)(10, below(Element.s, 50));

    noteAcceptButton.n = Button.sw - (0, 40);
    noteAcceptScroller.n = Scroller.s - (0, 40);

    drawObjects(Element, Button, Scroller, noteAcceptButton, noteAcceptScroller);

    link(inheritance)(pathStepY(Button.n, Element.s, 20));
    link(inheritance)(pathStepY(Scroller.n, Element.s, 20));
    link(dashedLink)(Button.methodStack.pict[0].w -- noteAcceptButton.n);
    link(dashedLink)(Scroller.methodStack.pict[0].w -- noteAcceptScroller.nw + (10, 0));
endfig;

end