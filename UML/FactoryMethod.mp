input metauml;
beginfig(1);
    % Set some styles
    iClass.iAttributeStack.top := 0;
    iClass.iAttributeStack.bottom := 0;
    iClass.foreColor := white;
    iInterface.foreColor := white;

    % Factory interface
    Interface.Factory("Factory")("+ Create(): T");
    classStereotypes.Factory("<<interface>>");
    ClassTemplate.T("T")(Factory);

    % Concrete factories
    Class.BrandFactory("BrandFactory")()("+ Create(): Button");
    Class.FancyFactory("FancyFactory")()("+ Create(): Button");

    BrandFactory.info.foreColor := (0.8, 0.8, 1);
    FancyFactory.info.foreColor := (1, 0.8, 0.8);

    % New style for name only button interface
    ClassInfoCopy.iProduct(iInterface);

    iProduct.iName.top := 10;
    iProduct.iName.bottom := 10;
    iProduct.iMethodStack.top := 0;
    iProduct.iMethodStack.bottom := 0;

    % Button interface
    EClass.Button(iProduct)("Button")()();
    classStereotypes.Button("<<interface>> ");

    % Concrete buttons
    ClassName.BlueButton("BlueButton");
    ClassName.GlossyButton("GlossyButton");

    BlueButton.info.foreColor := (0.8, 0.8, 1);
    GlossyButton.info.foreColor := (1, 0.8, 0.8);

    % Code notes
    Note.noteBlueButton("return BlueButton{};");
    Note.noteGlossyButton("return GlossyButton{};");


    % Alignments
    leftToRight.midy(40)(BrandFactory, Button, FancyFactory);
    topToBottom.midx(30)(Factory, Button);

    centered_allign_top(BlueButton, GlossyButton)(10, below(Button.s, 30));

    topToBottom.right(10)(BrandFactory, noteBlueButton);
    topToBottom.left(10)(FancyFactory, noteGlossyButton);


    drawObjects(Factory, T, BrandFactory, FancyFactory, Button, BlueButton, GlossyButton, noteBlueButton, noteGlossyButton);

    % Factory links
    link(inheritance)(pathStepY(BrandFactory.n, Factory.s, 15));
    link(inheritance)(pathStepY(FancyFactory.n, Factory.s, 15));

    item(iStereo)("<<bind>>")(obj.n = .65[BrandFactory.n,Factory.s]);
    item(iStereo)("<T -> Button>")(obj.nw = .45[BrandFactory.n,Factory.s]);
    item(iStereo)("<<bind>>")(obj.n = .65[FancyFactory.n,Factory.s]);
    item(iStereo)("<T -> Button>")(obj.ne = .45[FancyFactory.n,Factory.s]);

    % Button links
    clink(dependency)(BrandFactory, Button);
    clink(dependency)(FancyFactory, Button);
    
    link(inheritance)(pathStepY(BlueButton.n, Button.s, 5));
    link(inheritance)(pathStepY(GlossyButton.n, Button.s, 5));

    % Note links
    clink(dashedLink)(BrandFactory.methodStack.pict[0], noteBlueButton);
    clink(dashedLink)(FancyFactory.methodStack.pict[0], noteGlossyButton);
endfig;
end