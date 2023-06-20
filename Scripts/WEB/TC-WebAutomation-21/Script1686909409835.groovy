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

WebUI.navigateToUrl('https://www.fnp.com/gifts-lp?gclid=CjwKCAjwkLCkBhA9EiwAka9QRj0TkM-nJPrJbHCXODXaGabtng444kmzjaywke8tfuejg-8FJIWOfRoC3uAQAvD_BwE')

WebUI.click(findTestObject('Object Repository/fnp/Page_Online Gifts Delivery  UPTO 25OFF  Sur_eebf9e/img_Tomorrow_product-card_current-prod-img__a55c27'))

WebUI.switchToWindowTitle('Buy/Send Personalised Love Affair LED Cushion Online- FNP')

WebUI.setText(findTestObject('Object Repository/fnp/Page_BuySend Personalised Love Affair LED C_cf7bbc/input_Know More_autocomplete-input'), 
    '272020')

WebUI.click(findTestObject('Object Repository/fnp/Page_BuySend Personalised Love Affair LED C_cf7bbc/span_ADD TO CART'))

WebUI.setText(findTestObject('Object Repository/fnp/Page_BuySend Personalised Love Affair LED C_cf7bbc/input_Know More_autocomplete-input'), 
    '27')

WebUI.click(findTestObject('Object Repository/fnp/Page_BuySend Personalised Love Affair LED C_cf7bbc/span_271001, Gonda, Uttar Pradesh'))

WebUI.rightClick(findTestObject('Object Repository/fnp/Page_BuySend Personalised Love Affair LED C_cf7bbc/div_Pincode 271001'))

WebUI.click(findTestObject('Object Repository/fnp/Page_BuySend Personalised Love Affair LED C_cf7bbc/span_ADD TO CART'))

WebUI.closeBrowser()

