<?xml version="1.0" encoding="UTF-8"?>
<WebElementEntity>
   <description></description>
   <name>div_context httpschema.org,  type BlogPosti_d8cf22</name>
   <tag></tag>
   <elementGuidId>bd3c4e9c-51ee-4ebf-b1c0-558b92e71e45</elementGuidId>
   <selectorCollection>
      <entry>
         <key>CSS</key>
         <value>div.post</value>
      </entry>
      <entry>
         <key>XPATH</key>
         <value>//div[@id='Blog1']/div/article/div/div</value>
      </entry>
   </selectorCollection>
   <selectorMethod>XPATH</selectorMethod>
   <useRalativeImagePath>true</useRalativeImagePath>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>tag</name>
      <type>Main</type>
      <value>div</value>
      <webElementGuid>3a40fb40-da0c-457f-9f2f-0f5a0417e1a3</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>class</name>
      <type>Main</type>
      <value>post</value>
      <webElementGuid>06d74ea5-9b8d-42b7-abdd-4d6a4220c8f1</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>text</name>
      <type>Main</type>
      <value>{
  &quot;@context&quot;: &quot;http://schema.org&quot;,
  &quot;@type&quot;: &quot;BlogPosting&quot;,
  &quot;mainEntityOfPage&quot;: {
    &quot;@type&quot;: &quot;WebPage&quot;,
    &quot;@id&quot;: &quot;http://www.techlistic.com/p/selenium-practice-form.html&quot;
  },
  &quot;headline&quot;: &quot;Demo Automation Practice Form&quot;,&quot;description&quot;: &quot;AUTOMATION PRACTICE FORM First name: &amp;#160; Last name: &amp;#160; Gender &amp;#160; &amp;#160; Male&amp;#160;&amp;#160;&amp;#160;&amp;#160; &amp;#160;Female Years of Experience &amp;#160; 1&amp;#160;&amp;#160;&amp;#160;&amp;#160; 2&amp;#160;&amp;#160;&amp;#160;&amp;#160; 3 &amp;#160;&amp;#160;&amp;#160; 4 &amp;#160;&amp;#160;&amp;#160; 5 &amp;#160;&amp;#160;&amp;#160; 6 &amp;#160;&amp;#160;&amp;#160; ...&quot;,&quot;datePublished&quot;: &quot;2020-07-14T11:39:00+05:30&quot;,
  &quot;dateModified&quot;: &quot;2023-06-13T13:44:37+05:30&quot;,&quot;image&quot;: {
         &quot;@type&quot;: &quot;ImageObject&quot;,&quot;url&quot;: &quot;https://lh3.googleusercontent.com/ULB6iBuCeTVvSjjjU1A-O8e9ZpVba6uvyhtiWRti_rBAs9yMYOFBujxriJRZ-A=w1200&quot;,
            &quot;height&quot;: 348,
            &quot;width&quot;: 1200},&quot;publisher&quot;: {
         &quot;@type&quot;: &quot;Organization&quot;,
         &quot;name&quot;: &quot;Blogger&quot;,
         &quot;logo&quot;: {
         &quot;@type&quot;: &quot;ImageObject&quot;,
         &quot;url&quot;: &quot;https://lh3.googleusercontent.com/ULB6iBuCeTVvSjjjU1A-O8e9ZpVba6uvyhtiWRti_rBAs9yMYOFBujxriJRZ-A=h60&quot;,
         &quot;width&quot;: 206,
         &quot;height&quot;: 60
         }
         },&quot;author&quot;: {
    &quot;@type&quot;: &quot;Person&quot;,
    &quot;name&quot;: &quot;Vaneesh Behl&quot;
  }
}Demo Automation Practice Form



Get link

Facebook

Twitter

Pinterest

Email

Other AppsAUTOMATION PRACTICE FORMFirst name: Last name: Gender   Male     FemaleYears of Experience 1    2    3    4    5    6    7Date: Profession  Manual Tester     Automation TesterAutomation Tools   UFT   Protractor   Selenium WebdriverContinentsAsiaEuropeAfricaAustraliaSouth AmericaNorth AmericaAntarticaSelenium CommandsBrowser CommandsNavigation CommandsSwitch CommandsWait CommandsWebElement CommandsUpload Image Download File Click here to Download FileButtonHow to Automate 'Practice Form' with Selenium WebDriverThis is a demo web form that contains all the form elements which are mostly present on any web page. So, by automating this form you would learn all the WebDriver commands which are required to automate a web page. This web form contains the following web elements:LinksText boxesRadio buttonsDate PickerCheckboxesSelect boxMulti Select boxUpload Image buttonDownload linkSteps to Automate:Open this link - https://www.techlistic.com/p/selenium-practice-form.htmlEnter first and last name (textbox).Select gender (radio button).Select years of experience (radio button).Enter date.Select Profession (Checkbox).Select Automation tools you are familiar with (multiple checkboxes).Select Continent (Select box).Select multiple commands from a multi-select box.If you can handle Upload image then try it or leave this step.Click on the Download File link and handle the download file pop-up (leave it if you are a beginner).Click on Submit button.Try your own logic and automate it without any help. If you still find it difficult to automate then follow reference links. Example Code:package com.techlistic.tute;

import org.openqa.selenium.By;
import org.openqa.selenium.WebDriver;
import org.openqa.selenium.chrome.ChromeDriver;
import org.openqa.selenium.remote.DesiredCapabilities;
import org.openqa.selenium.Dimension;
import org.openqa.selenium.WebElement;
import org.openqa.selenium.interactions.Actions;
import org.openqa.selenium.support.ui.ExpectedConditions;
import org.openqa.selenium.support.ui.WebDriverWait;
import org.openqa.selenium.JavascriptExecutor;
import org.openqa.selenium.Alert;
import org.openqa.selenium.Keys;
import java.util.*;
import java.net.MalformedURLException;
import java.net.URL;

public class PracticeFormTest {

    // Main Method is the execution point of any Java Program
    public static void main(String[] args){
        // Initialize Webdriver Object
        System.setProperty(&quot;webdriver.chrome.driver&quot;, &quot;D:\\chromedriver.exe&quot;);
        WebDriver driver = new ChromeDriver();
        // Open URL
        driver.get(&quot;https://www.techlistic.com/p/selenium-practice-form.html&quot;);
        // Set Chrome window size
        driver.manage().window().setSize(new Dimension(1552, 840));
        // Enter Firstname
        driver.findElement(By.name(&quot;firstname&quot;)).click();
        driver.findElement(By.name(&quot;firstname&quot;)).sendKeys(&quot;Tom&quot;);
        // Set Lastname
        driver.findElement(By.name(&quot;lastname&quot;)).click();
        driver.findElement(By.name(&quot;lastname&quot;)).sendKeys(&quot;Wood&quot;);
        // Select Gender
        driver.findElement(By.id(&quot;sex-0&quot;)).click();
        // Select Experience
        driver.findElement(By.id(&quot;exp-4&quot;)).click();
        // Enter Date
        driver.findElement(By.id(&quot;datepicker&quot;)).click();
        driver.findElement(By.id(&quot;datepicker&quot;)).sendKeys(&quot;16-10-2020&quot;);
        // Select Profession
        driver.findElement(By.id(&quot;profession-1&quot;)).click();
        // Select Automation Tool
        driver.findElement(By.id(&quot;tool-2&quot;)).click();

        // Select Continent
        driver.findElement(By.id(&quot;continents&quot;)).click();
        WebElement dropdown = driver.findElement(By.id(&quot;continents&quot;));
        dropdown.findElement(By.xpath(&quot;//option[. = 'Europe']&quot;)).click();

        // Select Command
        WebElement dropdown = driver.findElement(By.id(&quot;selenium_commands&quot;));
        dropdown.findElement(By.xpath(&quot;//option[. = 'Browser Commands']&quot;)).click();

        // Scroll Vertical
        js.executeScript(&quot;window.scrollTo(0,675.5555419921875)&quot;);

        // Click Submit
        driver.findElement(By.id(&quot;submit&quot;)).click();
  }
}Popular Tutorials:Java TutorialSelenium with JavaSelenium with PythonSelenium with TestNGSelenium IDE (No Coding Required)Rest API Testing with PostmanPerformance Testing with JMeterMobile Testing with EmulatorYou may also like to read:50+ Selenium WebDriver Interview QuestionsTop 25 Selenium WebDriver Commands with JavaWhat are Java Class and Object?What is an Actions Class in Selenium?What is Robot Class in Selenium?How to take partial screenshots with Selenium?Checklist and Best Guidelines to Test a WebsiteYou may also like Selenium Practice Exercises with example code:Automate Amazon like E-Commerce website with SeleniumAutomate GoDaddy website features with SeleniumAutomate Google search with SeleniumHow do Find all the broken links on a website with SeleniumHow to Handle Static and Dynamic Web Tables with SeleniumAutomate Handling Multiple Browser Tabs with SeleniumAutomate Upload and Download files with SeleniumAutomate GoDaddy.com with Selenium &lt;&lt; Previous | Next  >> Automate Google SearchFollow TechlisticYouTube
Channel
|
Facebook
Page
|
Telegram
Channel
|
Quora SpaceFeel free to ask queries or share your thoughts in comments or email us.



Get link

Facebook

Twitter

Pinterest

Email

Other Apps
    
      
                  .AR_2 .ob_what a:after {content: &quot;&quot;;;;background-image: url('https://widgets.outbrain.com/images/widgetIcons/achoice.svg');background-size:100% 100%;width:12px;height:12px;padding-left:4px;display:inline-block;background-repeat:no-repeat;background-position:right center;border-left:1px solid #999;}
    .AR_2.ob-widget .ob_what{direction:ltr;clear:both;padding:5px 10px 0px;}
.AR_2.ob-widget .ob_what a{color:#757575;font-size:11px;font-family:arial;text-decoration: none;}
.AR_2.ob-widget .ob_what.ob-hover:hover a{text-decoration: underline;}
.AR_2.ob-widget .ob_amelia,
.AR_2.ob-widget .ob_amelia_covid,
.AR_2.ob-widget .ob_logo,
.AR_2.ob-widget .ob_feed_logo,
.AR_2.ob-widget .ob_sfeed_logo,
.AR_2.ob-widget .ob_text_logo{vertical-align:baseline !important;display:inline-block;vertical-align:text-bottom;padding:0px 5px;box-sizing:content-box;-moz-box-sizing:content-box;-webkit-box-sizing:content-box;}.AR_2.ob-widget .ob_logo{background:url('https://widgets.outbrain.com/images/widgetIcons/ob_logo_67x12.png') no-repeat center top;width:67px;height:12px;}  .AR_2.ob-widget .ob_what{text-align:right;}
  .AR_2.ob-widget .ob_what.ob_what_resp{position: static;float:  right;padding: 0px;font-weight: normal;flex:none;}

.AR_2.ob-widget {width:auto;min-width:0px;}
.AR_2, .AR_2 *{-webkit-box-sizing: content-box;-moz-box-sizing: content-box;box-sizing: content-box;}
.AR_2.ob-widget.ob-feed-layout{min-width:0px;clear:both;}
.AR_2.ob-widget.ob-feed-layout .ob-dynamic-rec-container{max-width:none;}
.AR_2.ob-widget.ob-feed-layout .ob-rec-source{overflow:hidden;} .AR_2.ob-widget.ob-feed-layout .ob-widget-header .ob_what{padding:5px 0;}
.AR_2.ob-widget .ob-dynamic-rec-container ~ .ob-dynamic-rec-container{margin: 0px;}

.AR_2.ob-widget.ob-feed-layout .ob-widget-header{direction:ltr;display: flex;justify-content: space-between;align-items: center;}
.AR_2.ob-widget.ob-feed-layout .ob-widget-header {font-weight:normal;}
    .AR_2.ob-widget.ob-feed-layout .ob-cards .OUTBRAIN:not(:last-child){ margin-bottom: 24px !important;}
.AR_2.ob-widget.ob-feed-layout .ob-cards .OUTBRAIN:last-child .ob-grid-layout .ob-widget-items-container {margin-bottom: 0;}
      
.AR_2.ob-feed-layout .ob-widget-header {font-family:inherit;font-size:18px;color:black;margin:8px 0px 12px 0px}
.AR_2.ob-feed-layout .ob-unit.ob-rec-image-container span.ob-rec-label {background-color:#666;}
.AR_2.ob-feed-layout .ob-widget-section.ob-first .ob-widget-items-container.ob-multi-row {padding-top:12px;}
.AR_2.ob-feed-layout {margin:0px;}
    
    
    .ob-smartfeed-wrapper .ob-widget.ob-feed-layout {   padding: 0 3px; } .ob-smartfeed-wrapper .OUTBRAIN:first-child {   margin-top: 0; } .ob-smartfeed-wrapper .OUTBRAIN {   border: 0; }  .ob-cards .OUTBRAIN .ob-dynamic-rec-container {   border: 1px solid #d9dada; }
    
    
        Sponsored Content
    
		Recommended by
	
    

        
            
                
                    
                
            
            
                
        
    
        
    
            
        
        
            
        
    
        
            
        
    
		
    
		
			
		
	
    
    
            
</value>
      <webElementGuid>afe538e1-4b67-4bb4-af65-cb300c917f2c</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath</name>
      <type>Main</type>
      <value>id(&quot;Blog1&quot;)/div[@class=&quot;blog-posts hfeed container&quot;]/article[@class=&quot;post-outer-container&quot;]/div[@class=&quot;post-outer&quot;]/div[@class=&quot;post&quot;]</value>
      <webElementGuid>59401ece-55d1-4861-bcf5-403f1530e289</webElementGuid>
   </webElementProperties>
   <webElementXpaths>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:idRelative</name>
      <type>Main</type>
      <value>//div[@id='Blog1']/div/article/div/div</value>
      <webElementGuid>ab5af18b-4db0-47c1-a223-c727eeca61a5</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:neighbor</name>
      <type>Main</type>
      <value>(.//*[normalize-space(text()) and normalize-space(.)='report this ad'])[3]/following::div[5]</value>
      <webElementGuid>7882659c-6687-4b9d-8a52-a98148bc8d29</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:neighbor</name>
      <type>Main</type>
      <value>(.//*[normalize-space(text()) and normalize-space(.)='VB with MS-EXCEL'])[1]/following::div[5]</value>
      <webElementGuid>88231c46-cb1f-493e-aec6-83e80f35862e</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:position</name>
      <type>Main</type>
      <value>//article/div/div</value>
      <webElementGuid>057988ec-de9e-4206-bce2-5fca3b110c5d</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:customAttributes</name>
      <type>Main</type>
      <value>//div[(text() = concat(&quot;{
  &quot;@context&quot;: &quot;http://schema.org&quot;,
  &quot;@type&quot;: &quot;BlogPosting&quot;,
  &quot;mainEntityOfPage&quot;: {
    &quot;@type&quot;: &quot;WebPage&quot;,
    &quot;@id&quot;: &quot;http://www.techlistic.com/p/selenium-practice-form.html&quot;
  },
  &quot;headline&quot;: &quot;Demo Automation Practice Form&quot;,&quot;description&quot;: &quot;AUTOMATION PRACTICE FORM First name: &amp;#160; Last name: &amp;#160; Gender &amp;#160; &amp;#160; Male&amp;#160;&amp;#160;&amp;#160;&amp;#160; &amp;#160;Female Years of Experience &amp;#160; 1&amp;#160;&amp;#160;&amp;#160;&amp;#160; 2&amp;#160;&amp;#160;&amp;#160;&amp;#160; 3 &amp;#160;&amp;#160;&amp;#160; 4 &amp;#160;&amp;#160;&amp;#160; 5 &amp;#160;&amp;#160;&amp;#160; 6 &amp;#160;&amp;#160;&amp;#160; ...&quot;,&quot;datePublished&quot;: &quot;2020-07-14T11:39:00+05:30&quot;,
  &quot;dateModified&quot;: &quot;2023-06-13T13:44:37+05:30&quot;,&quot;image&quot;: {
         &quot;@type&quot;: &quot;ImageObject&quot;,&quot;url&quot;: &quot;https://lh3.googleusercontent.com/ULB6iBuCeTVvSjjjU1A-O8e9ZpVba6uvyhtiWRti_rBAs9yMYOFBujxriJRZ-A=w1200&quot;,
            &quot;height&quot;: 348,
            &quot;width&quot;: 1200},&quot;publisher&quot;: {
         &quot;@type&quot;: &quot;Organization&quot;,
         &quot;name&quot;: &quot;Blogger&quot;,
         &quot;logo&quot;: {
         &quot;@type&quot;: &quot;ImageObject&quot;,
         &quot;url&quot;: &quot;https://lh3.googleusercontent.com/ULB6iBuCeTVvSjjjU1A-O8e9ZpVba6uvyhtiWRti_rBAs9yMYOFBujxriJRZ-A=h60&quot;,
         &quot;width&quot;: 206,
         &quot;height&quot;: 60
         }
         },&quot;author&quot;: {
    &quot;@type&quot;: &quot;Person&quot;,
    &quot;name&quot;: &quot;Vaneesh Behl&quot;
  }
}Demo Automation Practice Form



Get link

Facebook

Twitter

Pinterest

Email

Other AppsAUTOMATION PRACTICE FORMFirst name: Last name: Gender   Male     FemaleYears of Experience 1    2    3    4    5    6    7Date: Profession  Manual Tester     Automation TesterAutomation Tools   UFT   Protractor   Selenium WebdriverContinentsAsiaEuropeAfricaAustraliaSouth AmericaNorth AmericaAntarticaSelenium CommandsBrowser CommandsNavigation CommandsSwitch CommandsWait CommandsWebElement CommandsUpload Image Download File Click here to Download FileButtonHow to Automate &quot; , &quot;'&quot; , &quot;Practice Form&quot; , &quot;'&quot; , &quot; with Selenium WebDriverThis is a demo web form that contains all the form elements which are mostly present on any web page. So, by automating this form you would learn all the WebDriver commands which are required to automate a web page. This web form contains the following web elements:LinksText boxesRadio buttonsDate PickerCheckboxesSelect boxMulti Select boxUpload Image buttonDownload linkSteps to Automate:Open this link - https://www.techlistic.com/p/selenium-practice-form.htmlEnter first and last name (textbox).Select gender (radio button).Select years of experience (radio button).Enter date.Select Profession (Checkbox).Select Automation tools you are familiar with (multiple checkboxes).Select Continent (Select box).Select multiple commands from a multi-select box.If you can handle Upload image then try it or leave this step.Click on the Download File link and handle the download file pop-up (leave it if you are a beginner).Click on Submit button.Try your own logic and automate it without any help. If you still find it difficult to automate then follow reference links. Example Code:package com.techlistic.tute;

import org.openqa.selenium.By;
import org.openqa.selenium.WebDriver;
import org.openqa.selenium.chrome.ChromeDriver;
import org.openqa.selenium.remote.DesiredCapabilities;
import org.openqa.selenium.Dimension;
import org.openqa.selenium.WebElement;
import org.openqa.selenium.interactions.Actions;
import org.openqa.selenium.support.ui.ExpectedConditions;
import org.openqa.selenium.support.ui.WebDriverWait;
import org.openqa.selenium.JavascriptExecutor;
import org.openqa.selenium.Alert;
import org.openqa.selenium.Keys;
import java.util.*;
import java.net.MalformedURLException;
import java.net.URL;

public class PracticeFormTest {

    // Main Method is the execution point of any Java Program
    public static void main(String[] args){
        // Initialize Webdriver Object
        System.setProperty(&quot;webdriver.chrome.driver&quot;, &quot;D:\\chromedriver.exe&quot;);
        WebDriver driver = new ChromeDriver();
        // Open URL
        driver.get(&quot;https://www.techlistic.com/p/selenium-practice-form.html&quot;);
        // Set Chrome window size
        driver.manage().window().setSize(new Dimension(1552, 840));
        // Enter Firstname
        driver.findElement(By.name(&quot;firstname&quot;)).click();
        driver.findElement(By.name(&quot;firstname&quot;)).sendKeys(&quot;Tom&quot;);
        // Set Lastname
        driver.findElement(By.name(&quot;lastname&quot;)).click();
        driver.findElement(By.name(&quot;lastname&quot;)).sendKeys(&quot;Wood&quot;);
        // Select Gender
        driver.findElement(By.id(&quot;sex-0&quot;)).click();
        // Select Experience
        driver.findElement(By.id(&quot;exp-4&quot;)).click();
        // Enter Date
        driver.findElement(By.id(&quot;datepicker&quot;)).click();
        driver.findElement(By.id(&quot;datepicker&quot;)).sendKeys(&quot;16-10-2020&quot;);
        // Select Profession
        driver.findElement(By.id(&quot;profession-1&quot;)).click();
        // Select Automation Tool
        driver.findElement(By.id(&quot;tool-2&quot;)).click();

        // Select Continent
        driver.findElement(By.id(&quot;continents&quot;)).click();
        WebElement dropdown = driver.findElement(By.id(&quot;continents&quot;));
        dropdown.findElement(By.xpath(&quot;//option[. = &quot; , &quot;'&quot; , &quot;Europe&quot; , &quot;'&quot; , &quot;]&quot;)).click();

        // Select Command
        WebElement dropdown = driver.findElement(By.id(&quot;selenium_commands&quot;));
        dropdown.findElement(By.xpath(&quot;//option[. = &quot; , &quot;'&quot; , &quot;Browser Commands&quot; , &quot;'&quot; , &quot;]&quot;)).click();

        // Scroll Vertical
        js.executeScript(&quot;window.scrollTo(0,675.5555419921875)&quot;);

        // Click Submit
        driver.findElement(By.id(&quot;submit&quot;)).click();
  }
}Popular Tutorials:Java TutorialSelenium with JavaSelenium with PythonSelenium with TestNGSelenium IDE (No Coding Required)Rest API Testing with PostmanPerformance Testing with JMeterMobile Testing with EmulatorYou may also like to read:50+ Selenium WebDriver Interview QuestionsTop 25 Selenium WebDriver Commands with JavaWhat are Java Class and Object?What is an Actions Class in Selenium?What is Robot Class in Selenium?How to take partial screenshots with Selenium?Checklist and Best Guidelines to Test a WebsiteYou may also like Selenium Practice Exercises with example code:Automate Amazon like E-Commerce website with SeleniumAutomate GoDaddy website features with SeleniumAutomate Google search with SeleniumHow do Find all the broken links on a website with SeleniumHow to Handle Static and Dynamic Web Tables with SeleniumAutomate Handling Multiple Browser Tabs with SeleniumAutomate Upload and Download files with SeleniumAutomate GoDaddy.com with Selenium &lt;&lt; Previous | Next  >> Automate Google SearchFollow TechlisticYouTube
Channel
|
Facebook
Page
|
Telegram
Channel
|
Quora SpaceFeel free to ask queries or share your thoughts in comments or email us.



Get link

Facebook

Twitter

Pinterest

Email

Other Apps
    
      
                  .AR_2 .ob_what a:after {content: &quot;&quot;;;;background-image: url(&quot; , &quot;'&quot; , &quot;https://widgets.outbrain.com/images/widgetIcons/achoice.svg&quot; , &quot;'&quot; , &quot;);background-size:100% 100%;width:12px;height:12px;padding-left:4px;display:inline-block;background-repeat:no-repeat;background-position:right center;border-left:1px solid #999;}
    .AR_2.ob-widget .ob_what{direction:ltr;clear:both;padding:5px 10px 0px;}
.AR_2.ob-widget .ob_what a{color:#757575;font-size:11px;font-family:arial;text-decoration: none;}
.AR_2.ob-widget .ob_what.ob-hover:hover a{text-decoration: underline;}
.AR_2.ob-widget .ob_amelia,
.AR_2.ob-widget .ob_amelia_covid,
.AR_2.ob-widget .ob_logo,
.AR_2.ob-widget .ob_feed_logo,
.AR_2.ob-widget .ob_sfeed_logo,
.AR_2.ob-widget .ob_text_logo{vertical-align:baseline !important;display:inline-block;vertical-align:text-bottom;padding:0px 5px;box-sizing:content-box;-moz-box-sizing:content-box;-webkit-box-sizing:content-box;}.AR_2.ob-widget .ob_logo{background:url(&quot; , &quot;'&quot; , &quot;https://widgets.outbrain.com/images/widgetIcons/ob_logo_67x12.png&quot; , &quot;'&quot; , &quot;) no-repeat center top;width:67px;height:12px;}  .AR_2.ob-widget .ob_what{text-align:right;}
  .AR_2.ob-widget .ob_what.ob_what_resp{position: static;float:  right;padding: 0px;font-weight: normal;flex:none;}

.AR_2.ob-widget {width:auto;min-width:0px;}
.AR_2, .AR_2 *{-webkit-box-sizing: content-box;-moz-box-sizing: content-box;box-sizing: content-box;}
.AR_2.ob-widget.ob-feed-layout{min-width:0px;clear:both;}
.AR_2.ob-widget.ob-feed-layout .ob-dynamic-rec-container{max-width:none;}
.AR_2.ob-widget.ob-feed-layout .ob-rec-source{overflow:hidden;} .AR_2.ob-widget.ob-feed-layout .ob-widget-header .ob_what{padding:5px 0;}
.AR_2.ob-widget .ob-dynamic-rec-container ~ .ob-dynamic-rec-container{margin: 0px;}

.AR_2.ob-widget.ob-feed-layout .ob-widget-header{direction:ltr;display: flex;justify-content: space-between;align-items: center;}
.AR_2.ob-widget.ob-feed-layout .ob-widget-header {font-weight:normal;}
    .AR_2.ob-widget.ob-feed-layout .ob-cards .OUTBRAIN:not(:last-child){ margin-bottom: 24px !important;}
.AR_2.ob-widget.ob-feed-layout .ob-cards .OUTBRAIN:last-child .ob-grid-layout .ob-widget-items-container {margin-bottom: 0;}
      
.AR_2.ob-feed-layout .ob-widget-header {font-family:inherit;font-size:18px;color:black;margin:8px 0px 12px 0px}
.AR_2.ob-feed-layout .ob-unit.ob-rec-image-container span.ob-rec-label {background-color:#666;}
.AR_2.ob-feed-layout .ob-widget-section.ob-first .ob-widget-items-container.ob-multi-row {padding-top:12px;}
.AR_2.ob-feed-layout {margin:0px;}
    
    
    .ob-smartfeed-wrapper .ob-widget.ob-feed-layout {   padding: 0 3px; } .ob-smartfeed-wrapper .OUTBRAIN:first-child {   margin-top: 0; } .ob-smartfeed-wrapper .OUTBRAIN {   border: 0; }  .ob-cards .OUTBRAIN .ob-dynamic-rec-container {   border: 1px solid #d9dada; }
    
    
        Sponsored Content
    
		Recommended by
	
    

        
            
                
                    
                
            
            
                
        
    
        
    
            
        
        
            
        
    
        
            
        
    
		
    
		
			
		
	
    
    
            
&quot;) or . = concat(&quot;{
  &quot;@context&quot;: &quot;http://schema.org&quot;,
  &quot;@type&quot;: &quot;BlogPosting&quot;,
  &quot;mainEntityOfPage&quot;: {
    &quot;@type&quot;: &quot;WebPage&quot;,
    &quot;@id&quot;: &quot;http://www.techlistic.com/p/selenium-practice-form.html&quot;
  },
  &quot;headline&quot;: &quot;Demo Automation Practice Form&quot;,&quot;description&quot;: &quot;AUTOMATION PRACTICE FORM First name: &amp;#160; Last name: &amp;#160; Gender &amp;#160; &amp;#160; Male&amp;#160;&amp;#160;&amp;#160;&amp;#160; &amp;#160;Female Years of Experience &amp;#160; 1&amp;#160;&amp;#160;&amp;#160;&amp;#160; 2&amp;#160;&amp;#160;&amp;#160;&amp;#160; 3 &amp;#160;&amp;#160;&amp;#160; 4 &amp;#160;&amp;#160;&amp;#160; 5 &amp;#160;&amp;#160;&amp;#160; 6 &amp;#160;&amp;#160;&amp;#160; ...&quot;,&quot;datePublished&quot;: &quot;2020-07-14T11:39:00+05:30&quot;,
  &quot;dateModified&quot;: &quot;2023-06-13T13:44:37+05:30&quot;,&quot;image&quot;: {
         &quot;@type&quot;: &quot;ImageObject&quot;,&quot;url&quot;: &quot;https://lh3.googleusercontent.com/ULB6iBuCeTVvSjjjU1A-O8e9ZpVba6uvyhtiWRti_rBAs9yMYOFBujxriJRZ-A=w1200&quot;,
            &quot;height&quot;: 348,
            &quot;width&quot;: 1200},&quot;publisher&quot;: {
         &quot;@type&quot;: &quot;Organization&quot;,
         &quot;name&quot;: &quot;Blogger&quot;,
         &quot;logo&quot;: {
         &quot;@type&quot;: &quot;ImageObject&quot;,
         &quot;url&quot;: &quot;https://lh3.googleusercontent.com/ULB6iBuCeTVvSjjjU1A-O8e9ZpVba6uvyhtiWRti_rBAs9yMYOFBujxriJRZ-A=h60&quot;,
         &quot;width&quot;: 206,
         &quot;height&quot;: 60
         }
         },&quot;author&quot;: {
    &quot;@type&quot;: &quot;Person&quot;,
    &quot;name&quot;: &quot;Vaneesh Behl&quot;
  }
}Demo Automation Practice Form



Get link

Facebook

Twitter

Pinterest

Email

Other AppsAUTOMATION PRACTICE FORMFirst name: Last name: Gender   Male     FemaleYears of Experience 1    2    3    4    5    6    7Date: Profession  Manual Tester     Automation TesterAutomation Tools   UFT   Protractor   Selenium WebdriverContinentsAsiaEuropeAfricaAustraliaSouth AmericaNorth AmericaAntarticaSelenium CommandsBrowser CommandsNavigation CommandsSwitch CommandsWait CommandsWebElement CommandsUpload Image Download File Click here to Download FileButtonHow to Automate &quot; , &quot;'&quot; , &quot;Practice Form&quot; , &quot;'&quot; , &quot; with Selenium WebDriverThis is a demo web form that contains all the form elements which are mostly present on any web page. So, by automating this form you would learn all the WebDriver commands which are required to automate a web page. This web form contains the following web elements:LinksText boxesRadio buttonsDate PickerCheckboxesSelect boxMulti Select boxUpload Image buttonDownload linkSteps to Automate:Open this link - https://www.techlistic.com/p/selenium-practice-form.htmlEnter first and last name (textbox).Select gender (radio button).Select years of experience (radio button).Enter date.Select Profession (Checkbox).Select Automation tools you are familiar with (multiple checkboxes).Select Continent (Select box).Select multiple commands from a multi-select box.If you can handle Upload image then try it or leave this step.Click on the Download File link and handle the download file pop-up (leave it if you are a beginner).Click on Submit button.Try your own logic and automate it without any help. If you still find it difficult to automate then follow reference links. Example Code:package com.techlistic.tute;

import org.openqa.selenium.By;
import org.openqa.selenium.WebDriver;
import org.openqa.selenium.chrome.ChromeDriver;
import org.openqa.selenium.remote.DesiredCapabilities;
import org.openqa.selenium.Dimension;
import org.openqa.selenium.WebElement;
import org.openqa.selenium.interactions.Actions;
import org.openqa.selenium.support.ui.ExpectedConditions;
import org.openqa.selenium.support.ui.WebDriverWait;
import org.openqa.selenium.JavascriptExecutor;
import org.openqa.selenium.Alert;
import org.openqa.selenium.Keys;
import java.util.*;
import java.net.MalformedURLException;
import java.net.URL;

public class PracticeFormTest {

    // Main Method is the execution point of any Java Program
    public static void main(String[] args){
        // Initialize Webdriver Object
        System.setProperty(&quot;webdriver.chrome.driver&quot;, &quot;D:\\chromedriver.exe&quot;);
        WebDriver driver = new ChromeDriver();
        // Open URL
        driver.get(&quot;https://www.techlistic.com/p/selenium-practice-form.html&quot;);
        // Set Chrome window size
        driver.manage().window().setSize(new Dimension(1552, 840));
        // Enter Firstname
        driver.findElement(By.name(&quot;firstname&quot;)).click();
        driver.findElement(By.name(&quot;firstname&quot;)).sendKeys(&quot;Tom&quot;);
        // Set Lastname
        driver.findElement(By.name(&quot;lastname&quot;)).click();
        driver.findElement(By.name(&quot;lastname&quot;)).sendKeys(&quot;Wood&quot;);
        // Select Gender
        driver.findElement(By.id(&quot;sex-0&quot;)).click();
        // Select Experience
        driver.findElement(By.id(&quot;exp-4&quot;)).click();
        // Enter Date
        driver.findElement(By.id(&quot;datepicker&quot;)).click();
        driver.findElement(By.id(&quot;datepicker&quot;)).sendKeys(&quot;16-10-2020&quot;);
        // Select Profession
        driver.findElement(By.id(&quot;profession-1&quot;)).click();
        // Select Automation Tool
        driver.findElement(By.id(&quot;tool-2&quot;)).click();

        // Select Continent
        driver.findElement(By.id(&quot;continents&quot;)).click();
        WebElement dropdown = driver.findElement(By.id(&quot;continents&quot;));
        dropdown.findElement(By.xpath(&quot;//option[. = &quot; , &quot;'&quot; , &quot;Europe&quot; , &quot;'&quot; , &quot;]&quot;)).click();

        // Select Command
        WebElement dropdown = driver.findElement(By.id(&quot;selenium_commands&quot;));
        dropdown.findElement(By.xpath(&quot;//option[. = &quot; , &quot;'&quot; , &quot;Browser Commands&quot; , &quot;'&quot; , &quot;]&quot;)).click();

        // Scroll Vertical
        js.executeScript(&quot;window.scrollTo(0,675.5555419921875)&quot;);

        // Click Submit
        driver.findElement(By.id(&quot;submit&quot;)).click();
  }
}Popular Tutorials:Java TutorialSelenium with JavaSelenium with PythonSelenium with TestNGSelenium IDE (No Coding Required)Rest API Testing with PostmanPerformance Testing with JMeterMobile Testing with EmulatorYou may also like to read:50+ Selenium WebDriver Interview QuestionsTop 25 Selenium WebDriver Commands with JavaWhat are Java Class and Object?What is an Actions Class in Selenium?What is Robot Class in Selenium?How to take partial screenshots with Selenium?Checklist and Best Guidelines to Test a WebsiteYou may also like Selenium Practice Exercises with example code:Automate Amazon like E-Commerce website with SeleniumAutomate GoDaddy website features with SeleniumAutomate Google search with SeleniumHow do Find all the broken links on a website with SeleniumHow to Handle Static and Dynamic Web Tables with SeleniumAutomate Handling Multiple Browser Tabs with SeleniumAutomate Upload and Download files with SeleniumAutomate GoDaddy.com with Selenium &lt;&lt; Previous | Next  >> Automate Google SearchFollow TechlisticYouTube
Channel
|
Facebook
Page
|
Telegram
Channel
|
Quora SpaceFeel free to ask queries or share your thoughts in comments or email us.



Get link

Facebook

Twitter

Pinterest

Email

Other Apps
    
      
                  .AR_2 .ob_what a:after {content: &quot;&quot;;;;background-image: url(&quot; , &quot;'&quot; , &quot;https://widgets.outbrain.com/images/widgetIcons/achoice.svg&quot; , &quot;'&quot; , &quot;);background-size:100% 100%;width:12px;height:12px;padding-left:4px;display:inline-block;background-repeat:no-repeat;background-position:right center;border-left:1px solid #999;}
    .AR_2.ob-widget .ob_what{direction:ltr;clear:both;padding:5px 10px 0px;}
.AR_2.ob-widget .ob_what a{color:#757575;font-size:11px;font-family:arial;text-decoration: none;}
.AR_2.ob-widget .ob_what.ob-hover:hover a{text-decoration: underline;}
.AR_2.ob-widget .ob_amelia,
.AR_2.ob-widget .ob_amelia_covid,
.AR_2.ob-widget .ob_logo,
.AR_2.ob-widget .ob_feed_logo,
.AR_2.ob-widget .ob_sfeed_logo,
.AR_2.ob-widget .ob_text_logo{vertical-align:baseline !important;display:inline-block;vertical-align:text-bottom;padding:0px 5px;box-sizing:content-box;-moz-box-sizing:content-box;-webkit-box-sizing:content-box;}.AR_2.ob-widget .ob_logo{background:url(&quot; , &quot;'&quot; , &quot;https://widgets.outbrain.com/images/widgetIcons/ob_logo_67x12.png&quot; , &quot;'&quot; , &quot;) no-repeat center top;width:67px;height:12px;}  .AR_2.ob-widget .ob_what{text-align:right;}
  .AR_2.ob-widget .ob_what.ob_what_resp{position: static;float:  right;padding: 0px;font-weight: normal;flex:none;}

.AR_2.ob-widget {width:auto;min-width:0px;}
.AR_2, .AR_2 *{-webkit-box-sizing: content-box;-moz-box-sizing: content-box;box-sizing: content-box;}
.AR_2.ob-widget.ob-feed-layout{min-width:0px;clear:both;}
.AR_2.ob-widget.ob-feed-layout .ob-dynamic-rec-container{max-width:none;}
.AR_2.ob-widget.ob-feed-layout .ob-rec-source{overflow:hidden;} .AR_2.ob-widget.ob-feed-layout .ob-widget-header .ob_what{padding:5px 0;}
.AR_2.ob-widget .ob-dynamic-rec-container ~ .ob-dynamic-rec-container{margin: 0px;}

.AR_2.ob-widget.ob-feed-layout .ob-widget-header{direction:ltr;display: flex;justify-content: space-between;align-items: center;}
.AR_2.ob-widget.ob-feed-layout .ob-widget-header {font-weight:normal;}
    .AR_2.ob-widget.ob-feed-layout .ob-cards .OUTBRAIN:not(:last-child){ margin-bottom: 24px !important;}
.AR_2.ob-widget.ob-feed-layout .ob-cards .OUTBRAIN:last-child .ob-grid-layout .ob-widget-items-container {margin-bottom: 0;}
      
.AR_2.ob-feed-layout .ob-widget-header {font-family:inherit;font-size:18px;color:black;margin:8px 0px 12px 0px}
.AR_2.ob-feed-layout .ob-unit.ob-rec-image-container span.ob-rec-label {background-color:#666;}
.AR_2.ob-feed-layout .ob-widget-section.ob-first .ob-widget-items-container.ob-multi-row {padding-top:12px;}
.AR_2.ob-feed-layout {margin:0px;}
    
    
    .ob-smartfeed-wrapper .ob-widget.ob-feed-layout {   padding: 0 3px; } .ob-smartfeed-wrapper .OUTBRAIN:first-child {   margin-top: 0; } .ob-smartfeed-wrapper .OUTBRAIN {   border: 0; }  .ob-cards .OUTBRAIN .ob-dynamic-rec-container {   border: 1px solid #d9dada; }
    
    
        Sponsored Content
    
		Recommended by
	
    

        
            
                
                    
                
            
            
                
        
    
        
    
            
        
        
            
        
    
        
            
        
    
		
    
		
			
		
	
    
    
            
&quot;))]</value>
      <webElementGuid>ae66af1e-293c-49fc-acfb-524976718c88</webElementGuid>
   </webElementXpaths>
</WebElementEntity>
