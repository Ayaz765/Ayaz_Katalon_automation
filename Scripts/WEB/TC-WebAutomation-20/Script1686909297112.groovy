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

WebUI.navigateToUrl('https://www.snapdeal.com/')

WebUI.click(findTestObject('Object Repository/snapdeal/Page_Shop Online for Men, Women  Kids Cloth_832cc7/span_Mobile  Accessories'))

WebUI.click(findTestObject('Object Repository/snapdeal/Page_Shop Online for Men, Women  Kids Cloth_832cc7/a_NBOX - Blue Silicon Plain Cases Compatibl_e3ad25'))

WebUI.switchToWindowTitle('NBOX - Blue Silicon Plain Cases Compatible For iPhone 13 ( Pack of 1 ) - Plain Back Covers Online at Low Prices | Snapdeal India')

WebUI.click(findTestObject('Object Repository/snapdeal/Page_NBOX - Blue Silicon Plain Cases Compat_6b044f/span_add to cart'))

WebUI.click(findTestObject('Object Repository/snapdeal/Page_/a_Proceed To Checkout'))

WebUI.closeBrowser()

