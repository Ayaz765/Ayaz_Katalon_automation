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

WebUI.navigateToUrl('https://www.telerik.com/support/demos')

WebUI.click(findTestObject('Object Repository/Telerik_product/Page_Telerik Product Demos, Examples and Tu_d36ce2/a_Web'))

WebUI.click(findTestObject('Object Repository/Telerik_product/Page_Telerik Product Demos, Examples and Tu_d36ce2/a_UI for jQuery demos'))

WebUI.switchToWindowTitle('jQuery Demos and Examples with HTML5/JavaScript Source Code | Kendo UI for jQuery')

WebUI.click(findTestObject('Object Repository/Telerik_product/Page_jQuery Demos and Examples with HTML5Ja_1f6872/a_cls-1         fill gray              barc_89a118'))

WebUI.closeBrowser()

