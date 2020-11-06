input metauml;

beginfig(1);
    Interface.Visitable("Visitable")
              ("+Accept(v:Visitor)");
    classStereotypes.Visitable("<<interface>>");
    Visitable.info.foreColor := white;

    Class.Button("Button")()
                    ("+Accept(v:Visitor)");
    Class.Scroller("Scroller")()
                  ("+Accept(v:Visitor)");

    Note.noteAcceptButton("v->Visit_Button(self)");
    Note.noteAcceptScroller("v->Visit_Scroller(self)");

    centered_allign_top(Button, Scroller)(10, below(Visitable.s, 25));

    noteAcceptButton.n = Button.sw - (0, 10);
    noteAcceptScroller.n = Scroller.s - (0, 10);

    drawObjects(Visitable, Button, Scroller, noteAcceptButton, noteAcceptScroller);

    link(inheritance)(pathStepY(Button.n, Visitable.s, 5));
    link(inheritance)(pathStepY(Scroller.n, Visitable.s, 5));
    link(dashedLink)(Button.methodStack.pict[0].w -- noteAcceptButton.n);
    link(dashedLink)(Scroller.methodStack.pict[0].w -- noteAcceptScroller.nw + (10, 0));
endfig;

end