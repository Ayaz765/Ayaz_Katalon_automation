import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testng.keyword.TestNGBuiltinKeywords as TestNGKW
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys

WebUI.openBrowser('')
WebUI.maximizeWindow()

WebUI.navigateToUrl('https://in.puma.com/in/en?msclkid=ad1854e2f9441fc61f72306c1dfbd952&utm_aud=OTH&utm_campaign=BS_BNG_SEA-TXT_Brand-Exact&utm_medium=BS&utm_obj=CONV&utm_source=BNG-SEA-TXT')

WebUI.click(findTestObject('Object Repository/Puma/Page_PUMA.COM  Forever Faster/span_Men'))

WebUI.click(findTestObject('Object Repository/Puma/Page_Mens Shoes, Clothing  Accessories - PU_1b48c1/img_8,999_w-full bg-puma-black-800 aspect-1-1'))

WebUI.click(findTestObject('Object Repository/Puma/Page_BMW M Motorsport SPEEDFUSION Unisex Sn_d3b603/button_Add to Cart'))

WebUI.click(findTestObject('Object Repository/Puma/Page_BMW M Motorsport SPEEDFUSION Unisex Sn_d3b603/p_Please select a size'))

WebUI.click(findTestObject('Object Repository/Puma/Page_BMW M Motorsport SPEEDFUSION Unisex Sn_d3b603/p_Please select a size'))

WebUI.click(findTestObject('Object Repository/Puma/Page_BMW M Motorsport SPEEDFUSION Unisex Sn_d3b603/span_Please select a size'))

WebUI.click(findTestObject('Object Repository/Puma/Page_BMW M Motorsport SPEEDFUSION Unisex Sn_d3b603/span_Size7'))

WebUI.click(findTestObject('Object Repository/Puma/Page_BMW M Motorsport SPEEDFUSION Unisex Sn_d3b603/span_Add to Cart'))

WebUI.click(findTestObject('Object Repository/Puma/Page_BMW M Motorsport SPEEDFUSION Unisex Sn_d3b603/a_View cart (1)  checkout'))

Thread.sleep(4500)

WebUI.takeScreenshotAsCheckpoint('full_view', FailureHandling.CONTINUE_ON_FAILURE)
WebUI.closeBrowser()

