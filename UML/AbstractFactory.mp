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

    % Abstract Factory interface
    Interface.AbstractFactory("AbstractFactory")
        ("+ Create(): Button",
        "+ Create(): Scroller",
        "+ Create(): ProductN");
    classStereotypes.AbstractFactory("<<interface>>");

    % Concrete factories
    Class.BrandFactory("BrandFactory")()("+ Create(): Button");
    Class.FancyFactory("FancyFactory")()("+ Create(): Button");

    BrandFactory.info.foreColor := (0.8, 0.8, 1);
    FancyFactory.info.foreColor := (1, 0.8, 0.8);

    % New style for name only interface
    ClassInfoCopy.iProduct(iInterface);

    iProduct.iName.top := 10;
    iProduct.iName.bottom := 10;
    iProduct.iMethodStack.top := 0;
    iProduct.iMethodStack.bottom := 0;

    % Products interfaces
    EClass.Button(iProduct)("Button")()();
    classStereotypes.Button("<<interface>> ");

    EClass.Scroller(iProduct)("Scroller")()();
    classStereotypes.Scroller("<<interface>> ");

    EClass.ProductN(iProduct)("ProductN")()();
    classStereotypes.ProductN("<<interface>> ");

    % New styles for factory products
    ClassInfoCopy.iBrand(iClassNameOnly);
    iBrand.foreColor := (0.8, 0.8, 1);

    ClassInfoCopy.iFancy(iClassNameOnly);
    iFancy.foreColor := (1, 0.8, 0.8);

    % Concrete buttons
    EClass.BlueButton(iBrand)("BlueButton")()();
    EClass.GlossyButton(iFancy)("GlossyButton")()();

    % Concrete scrollers
    EClass.BlueScroller(iBrand)("BlueScroller")()();
    EClass.GlossyScroller(iFancy)("GlossyScroller")()();

    % Concrete productNs
    EClass.BlueProductN(iBrand)("BlueProductN")()();
    EClass.GlossyProductN(iFancy)("GlossyProductN")()();

    % Alignments
    topToBottom.midx(40)(Factory, AbstractFactory);
    topToBottom.midx(50)(AbstractFactory, BrandFactory);

    topToBottom.left(15)(Button, Scroller, ProductN);
    leftToRight.bottom(50)(AbstractFactory, ProductN);

    topToBottom.midx(20)(BrandFactory, BlueProductN, BlueScroller, BlueButton);
    topToBottom.midx(20)(FancyFactory, GlossyProductN, GlossyScroller, GlossyButton);

    leftToRight.midy(50)(BlueScroller, GlossyScroller);


    drawObjects(Factory, T, AbstractFactory, Button, Scroller, ProductN, BrandFactory, FancyFactory, BlueProductN, GlossyProductN, BlueScroller, GlossyScroller, BlueButton, GlossyButton);

    % Factory links
    clink(dependency)(AbstractFactory, Factory);

    item(iStereo)("<<bind>>")(obj.nw = 0.1[Factory.s,AbstractFactory.n]);
    item(iStereo)("<T -> Button>")(obj.nw = .3[Factory.s,AbstractFactory.n]);
    item(iStereo)("<T -> Scroller>")(obj.nw = .5[Factory.s,AbstractFactory.n]);
    item(iStereo)("<T -> ProductN>")(obj.nw = .7[Factory.s,AbstractFactory.n]);

    % Create links
    link(dependency)(AbstractFactory.methodStack.pict[0].e -- Button.w);
    link(dependency)(AbstractFactory.methodStack.pict[1].e -- Scroller.w);
    link(dependency)(AbstractFactory.methodStack.pict[2].e -- ProductN.w);

    % Concrete Factory links
    link(inheritance)(pathStepY(BrandFactory.n, AbstractFactory.s, 15));
    link(inheritance)(pathStepY(FancyFactory.n, AbstractFactory.s, 15));

    % Concrete productN links
    path PathProductN;
    PathProductN = pathVertical(GlossyProductN.ne + (10, 5), ypart(ProductN.e)) -- ProductN.e;
    
    link(inheritance)(BlueProductN.n -- BlueProductN.n + (0, 5) -- PathProductN);
    link(inheritance)(GlossyProductN.n -- GlossyProductN.n + (0, 5) -- PathProductN);

    % Concrete scroller links
    path PathScroller;
    PathScroller = pathVertical(GlossyScroller.ne + (25, 5), ypart(Scroller.e)) -- Scroller.e;
    
    link(inheritance)(BlueScroller.n -- BlueScroller.n + (0, 5) -- PathScroller);
    link(inheritance)(GlossyScroller.n -- GlossyScroller.n + (0, 5) -- PathScroller);

    % Concrete button links
    path PathButton;
    PathButton = pathVertical(GlossyButton.ne + (40, 5), ypart(Button.e)) -- Button.e;
    
    link(inheritance)(BlueButton.n -- BlueButton.n + (0, 5) -- PathButton);
    link(inheritance)(GlossyButton.n -- GlossyButton.n + (0, 5) -- PathButton);
endfig;
end